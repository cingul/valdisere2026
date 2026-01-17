import streamlit as st
import cv2

import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
import numpy as np
import os
from moviepy import VideoFileClip, concatenate_videoclips
import tempfile
import time
import datetime

# --- Config ---
CASE_DIR = "case"
ANGIO_DIR = "angio"
ANON_DIR = os.path.join(CASE_DIR, "anonymized")
OUTPUT_DIR = "output_videos"
MODEL_PATH = 'blaze_face_short_range.tflite'

# Ensure directories exist
os.makedirs(ANON_DIR, exist_ok=True)
os.makedirs(OUTPUT_DIR, exist_ok=True)

st.set_page_config(page_title="STANDUP II - Video Studio", layout="wide")

# --- Face Anonymizer Logic (Tasks API) ---

# --- Face Anonymizer Logic (Tasks API + MoviePy) ---

# --- Face Anonymizer Logic (Tasks API + MoviePy + Temporal Persistence) ---

class FaceAnonymizer:
    def __init__(self, model_path, confidence=0.1, pixelation_factor=20, persistence_frames=30, padding=0.3):
        self.model_path = model_path
        self.confidence = confidence
        self.pixelation_factor = pixelation_factor
        self.persistence = persistence_frames
        self.padding = padding
        
        # State
        self.active_faces = [] # List of {'bbox': [x,y,w,h], 'ttl': int}
        
    def __enter__(self):
        base_options = python.BaseOptions(model_asset_path=self.model_path)
        options = vision.FaceDetectorOptions(base_options=base_options, min_detection_confidence=self.confidence)
        self.detector = vision.FaceDetector.create_from_options(options)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.detector.close()

    def process_frame(self, image_rgb):
        # 1. Prepare Image
        if not image_rgb.flags['WRITEABLE']:
            image_rgb = image_rgb.copy()
            
        h_img, w_img, c = image_rgb.shape
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=np.ascontiguousarray(image_rgb))
        
        # 2. Detect
        try:
            detection_result = self.detector.detect(mp_image)
            current_detections = []
            
            if detection_result.detections:
                for detection in detection_result.detections:
                    bbox = detection.bounding_box
                    current_detections.append([bbox.origin_x, bbox.origin_y, bbox.width, bbox.height])
            
            # 3. Update State (Temporal Persistence)
            if current_detections:
                self.active_faces = [{'bbox': box, 'ttl': self.persistence} for box in current_detections]
            else:
                for face in self.active_faces:
                    face['ttl'] -= 1
                self.active_faces = [f for f in self.active_faces if f['ttl'] > 0]
                
        except Exception as e:
            print(f"Detection Error: {e}")

        # 4. Pixelate Active Faces
        for face in self.active_faces:
            x, y, w_box, h_box = face['bbox']
            
            # Apply Padding
            pad_w = int(w_box * self.padding)
            pad_h = int(h_box * self.padding)
            
            x = x - pad_w
            y = y - pad_h
            w_box = w_box + (2 * pad_w)
            h_box = h_box + (2 * pad_h)
            
            # Clamp
            x = max(0, x)
            y = max(0, y)
            w_box = min(w_box, w_img - x)
            h_box = min(h_box, h_img - y)
            
            if w_box > 0 and h_box > 0:
                try:
                    roi = image_rgb[y:y+h_box, x:x+w_box]
                    
                    # Pixelation Logic
                    small_w = max(1, w_box // self.pixelation_factor)
                    small_h = max(1, h_box // self.pixelation_factor)
                    
                    small = cv2.resize(roi, (small_w, small_h), interpolation=cv2.INTER_LINEAR)
                    pixelated = cv2.resize(small, (w_box, h_box), interpolation=cv2.INTER_NEAREST)
                    image_rgb[y:y+h_box, x:x+w_box] = pixelated
                except Exception:
                    pass
                    
        return image_rgb

def process_video_anonymization(input_path, output_path, progress_bar, status_text):
    if not os.path.exists(MODEL_PATH):
        st.error(f"Model file {MODEL_PATH} not found!")
        return
    
    status_text.text("Initializing Smart Anonymizer (Tracking + Medium Pixelation)...")
    
    # Open Video Clip
    clip = VideoFileClip(input_path)
    
    # Use Context Manager for Anonymizer
    with FaceAnonymizer(MODEL_PATH) as anonymizer:
        
        # Transform
        anonymized_clip = clip.image_transform(anonymizer.process_frame)
        
        status_text.text("Processing... Applying Medium Pixelation.")
        anonymized_clip.write_videofile(
            output_path, 
            codec='libx264', 
            audio_codec='aac', 
            logger=None,
            ffmpeg_params=['-pix_fmt', 'yuv420p']
        )

    status_text.text("Processing complete!")
    progress_bar.progress(1.0)



# --- App UI ---

st.title("🏥 STANDUP II: Case Video Studio")

tab1, tab2 = st.tabs(["🕵️ Face Anonymizer", "🎬 Sequence Editor"])

# --- TAB 1: Anonymizer ---
with tab1:
    st.header("Anonymize Patient Videos")
    st.markdown("Select videos from the `case/` folder to pixelate faces.")
    
    if not os.path.exists(MODEL_PATH):
        st.error(f"⚠️ Model file `{MODEL_PATH}` not found in directory. Please check installation.")

    # List .MOV or .mp4 files in case/
    try:
        video_files = [f for f in os.listdir(CASE_DIR) if f.lower().endswith(('.mov', '.mp4'))]
        video_files.sort()
    except FileNotFoundError:
        st.error(f"Directory `{CASE_DIR}` not found.")
        video_files = []

    if not video_files:
        st.warning(f"No video files found in `{CASE_DIR}/`.")
    else:
        selected_video = st.selectbox("Select Video to Process", video_files)
        
        col1, col2 = st.columns(2)
        
        with col1:
            st.subheader("Source Info")
            video_path = os.path.join(CASE_DIR, selected_video)
            st.video(video_path)
        
        with col2:
            st.subheader("Action")
            output_filename = f"anon_{selected_video.rsplit('.', 1)[0]}.mp4"
            output_path = os.path.join(ANON_DIR, output_filename)
            
            if st.button("Run Process", type="primary"):
                progress_bar = st.progress(0)
                status_text = st.empty()
                
                try:
                    process_video_anonymization(video_path, output_path, progress_bar, status_text)
                    st.success(f"Saved to: `{output_path}`")
                    st.video(output_path)
                except Exception as e:
                    st.error(f"Error: {e}")

# --- TAB 2: Sequence Editor ---
with tab2:
    st.header("Interpolate Case & Angio Videos")
    st.markdown("Create a sequence mixing anonymized patient videos and angiograms.")

    st.subheader("Available Clips")
    
    col_a, col_b = st.columns(2)
    
    with col_a:
        st.markdown("**Patient Clips (Anonymized)**")
        try:
            anon_files = [f for f in os.listdir(ANON_DIR) if f.lower().endswith('.mp4')]
            anon_files.sort()
            st.write(anon_files)
        except Exception:
            st.write("No directory.")
            anon_files = []
        
    with col_b:
        st.markdown("**Angiogram Clips**")
        try:
            angio_files = [f for f in os.listdir(ANGIO_DIR) if f.lower().endswith(('.mov', '.mp4'))]
            angio_files.sort()
            st.write(angio_files)
        except Exception:
            st.write("No directory.")
            angio_files = []

    st.divider()
    
    st.subheader("Build Sequence")
    
    # Session state for sequence
    if 'sequence' not in st.session_state:
        st.session_state.sequence = []

    # --- Auto-Sequence Logic ---
    def get_timestamp(path):
        try:
            return datetime.datetime.fromtimestamp(os.path.getmtime(path))
        except:
            return datetime.datetime.min

    def is_file_ready(path):
        """Checks if a video file is completely written and valid."""
        try:
            # Quick check: can we read the duration?
            # context manager to ensure we close it immediately
            with VideoFileClip(path) as clip:
                 d = clip.duration
            return True
        except Exception:
            return False

    if st.button("✨ Auto-Generate Sequence (By Date)"):
        # 1. Patient Videos
        patient_items = []
        skipped_count = 0
        
        for f in anon_files:
            path = os.path.join(ANON_DIR, f)
            
            # Check if file is valid/ready
            if not is_file_ready(path):
                skipped_count += 1
                continue
                
            # Match back to original file to get timestamp?
            # Original: IMG_7532.MOV -> anon_IMG_7532.mp4
            # We need the timestamp of the ORIGINAL SOURCE in CASE_DIR
            original_name = f.replace("anon_", "").replace(".mp4", "")
            # Try to find extension
            orig_path = None
            for ext in ['.MOV', '.mp4', '.mov', '.MP4']:
                p = os.path.join(CASE_DIR, original_name + ext)
                if os.path.exists(p):
                    orig_path = p
                    break
            
            if orig_path:
                dt = get_timestamp(orig_path)
                patient_items.append({
                    "file": f,
                    "path": path,
                    "type": "Anonymized Patient",
                    "date": dt
                })
        
        if skipped_count > 0:
            st.warning(f"⚠️ Skipped {skipped_count} files that are still processing or incomplete. Please wait a few minutes and try again.")

        # 2. Angio Videos (Heuristic)
        # Dates: Jan 2, Dec 18, Dec 9, Dec 6, Dec 3 (Reverse Chronological)
        # Angio Files: Sorted List [0] = earliest recording time (7:38) = ? 
        # User said "Angio videos are in reverse chronological order".
        # Assuming listing them by name (Screen Recording ...) gives the order:
        # File 1 (7:38) -> Jan 2
        # ...
        # File 13 (8:44) -> Dec 3
        
        angio_dates = [
            datetime.datetime(2026, 1, 2, 12, 0),   # Jan 2
            datetime.datetime(2025, 12, 18, 12, 0), # Dec 18
            datetime.datetime(2025, 12, 9, 12, 0),  # Dec 9
            datetime.datetime(2025, 12, 6, 12, 0),  # Dec 6
            datetime.datetime(2025, 12, 3, 12, 0)   # Dec 3
        ]
        
        # Distribute 13 files into 5 buckets
        # 13 / 5 = 2.6 per bucket.
        # [3, 3, 3, 2, 2] ? 
        # Let's try to infer breaks or just split unevenly.
        angio_items = []
        
        # Sorted by name (timestamp in name)
        # [0..2] -> Jan 2
        # [3..5] -> Dec 18
        # [6..8] -> Dec 9
        # [9..10] -> Dec 6
        # [11..12] -> Dec 3
        
        # Logic: 
        # Jan 2: 3 files
        # Dec 18: 3 files
        # Dec 9: 3 files
        # Dec 6: 2 files
        # Dec 3: 2 files
        # Total: 13.
        
        chunk_sizes = [3, 3, 3, 2, 2] # Sum = 13
        
        current_idx = 0
        for i, count in enumerate(chunk_sizes):
            date_val = angio_dates[i]
            for _ in range(count):
                if current_idx < len(angio_files):
                    f = angio_files[current_idx]
                    path = os.path.join(ANGIO_DIR, f)
                    
                    if is_file_ready(path):
                        angio_items.append({
                            "file": f,
                            "path": path,
                            "type": "Angiogram",
                            "date": date_val
                        })
                    current_idx += 1

        # 3. Merge & Sort
        all_items = patient_items + angio_items
        all_items.sort(key=lambda x: x['date'])
        
        # 4. Update State
        st.session_state.sequence = all_items
        st.rerun()

    # Add Clip UI
    c1, c2, c3 = st.columns([2, 1, 1])
    with c1:
        source_type = st.radio("Source", ["Anonymized Patient", "Angiogram"], horizontal=True)
        if source_type == "Anonymized Patient":
             clip_options = anon_files
             base_dir_sel = ANON_DIR
        else:
             clip_options = angio_files
             base_dir_sel = ANGIO_DIR
        
        selected_clip_add = st.selectbox("Choose Clip", clip_options) if clip_options else None
    
    with c2:
        st.write("") 
        st.write("")
        if st.button("Add to Timeline") and selected_clip_add:
            st.session_state.sequence.append({
                "file": selected_clip_add,
                "path": os.path.join(base_dir_sel, selected_clip_add),
                "type": source_type,
                "date": datetime.datetime.now() # Fallback
            })

    # Show Timeline
    st.markdown("### 🎞️ Timeline")
    if st.session_state.sequence:
        for i, item in enumerate(st.session_state.sequence):
            col_t1, col_t2, col_t3 = st.columns([1, 4, 1])
            with col_t1:
                st.write(f"**#{i+1}**")
            with col_t2:
                date_str = item.get('date', datetime.datetime.min).strftime("%Y-%m-%d %H:%M")
                st.info(f"{item['type']}: {item['file']} \n\n 🕒 {date_str}")
            with col_t3:
                if st.button("❌", key=f"del_{i}"):
                    st.session_state.sequence.pop(i)
                    st.rerun()
    else:
        st.info("Timeline is empty. Add clips above.")

    st.divider()
    
    if st.button("Render Full Sequence", type="primary", disabled=not st.session_state.sequence):
        prog = st.progress(0)
        stat = st.empty()
        
        try:
            stat.text("Loading clips...")
            clips = []
            for item in st.session_state.sequence:
                # Use MoviePy to load
                clip = VideoFileClip(item["path"])
                # Resize to common height (e.g. 720p) to avoid errors? Or allow varying.
                # Ideally we resize to match the first clip or a standard size.
                if clip.w % 2 != 0: clip = clip.cropped(width=clip.w-1)
                if clip.h % 2 != 0: clip = clip.cropped(height=clip.h-1)
                
                clips.append(clip)
            
            stat.text("Concatenating...")
            final_clip = concatenate_videoclips(clips, method="compose") # compose handles different sizes better
            
            output_path_final = os.path.join(OUTPUT_DIR, "case_presentation_final.mp4")
            
            stat.text("Rendering... This may take a while.")
            final_clip.write_videofile(
                output_path_final, 
                codec='libx264', 
                audio_codec='aac', 
                ffmpeg_params=['-pix_fmt', 'yuv420p']
            )
            
            prog.progress(100)
            stat.success("Rendering Complete!")
            st.video(output_path_final)
            
        except Exception as e:
            st.error(f"Render Error: {e}")


