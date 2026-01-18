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
    def __init__(self, model_path, confidence=0.1, pixelation_factor=20, persistence_frames=30, padding=0.3, manual_overrides=None, manual_only=False):
        self.model_path = model_path
        self.confidence = confidence
        self.pixelation_factor = pixelation_factor
        self.persistence = persistence_frames
        self.padding = padding
        self.manual_overrides = manual_overrides if manual_overrides else []
        self.manual_only = manual_only
        
        # State
        self.active_faces = [] # List of {'bbox': [x,y,w,h], 'ttl': int}
        
    def __enter__(self):
        base_options = python.BaseOptions(model_asset_path=self.model_path)
        options = vision.FaceDetectorOptions(base_options=base_options, min_detection_confidence=self.confidence)
        self.detector = vision.FaceDetector.create_from_options(options)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.detector.close()

    def process_frame(self, image_rgb, t=0):
        # 1. Prepare Image
        if not image_rgb.flags['WRITEABLE']:
            image_rgb = image_rgb.copy()
            
        h_img, w_img, c = image_rgb.shape
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=np.ascontiguousarray(image_rgb))
        
        # 2. Check Manual Overrides
        # List of (start, end, x, y, w, h)
        override_faces = []
        is_overridden = False
        
        for entry in self.manual_overrides:
            # Check length to support both old (start, end) and new format
            if len(entry) == 2:
                # Legacy fallback (start, end) -> Default center
                s, e = entry
                if s <= t <= e:
                    # Default: Top 20% center
                    face_w = w_img // 4
                    face_h = w_img // 4
                    face_x = (w_img - face_w) // 2
                    face_y = h_img // 8
                    override_faces.append([face_x, face_y, face_w, face_h])
                    is_overridden = True
            elif len(entry) == 6:
                # Explicit (start, end, x, y, w, h)
                s, e, ox, oy, ow, oh = entry
                if s <= t <= e:
                    # Scale normalized coords (0-100) to pixels if needed, 
                    # OR assuming input is already in pixels? 
                    # Let's verify input. UI sends PIXELS.
                    override_faces.append([int(ox), int(oy), int(ow), int(oh)])
                    is_overridden = True
        
        # 3. Detect
        try:
            current_detections = []
            
            # Run detection normally (UNLESS manual_only is set)
            if not self.manual_only:
                detection_result = self.detector.detect(mp_image)
                
                if detection_result.detections:
                    for detection in detection_result.detections:
                        bbox = detection.bounding_box
                        current_detections.append([bbox.origin_x, bbox.origin_y, bbox.width, bbox.height])
            
            # Update State
            if current_detections:
                self.active_faces = [{'bbox': box, 'ttl': self.persistence} for box in current_detections]
            else:
                # Decay
                for face in self.active_faces:
                    face['ttl'] -= 1
                self.active_faces = [f for f in self.active_faces if f['ttl'] > 0]
                
            # Inject Overrides
            for obox in override_faces:
                self.active_faces.append({
                    'bbox': obox,
                    'ttl': 2 # Force blur this frame
                })

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

def process_video_anonymization(input_path, output_path, progress_bar, status_text, manual_overrides=None, manual_only=False):
    if not os.path.exists(MODEL_PATH):
        st.error(f"Model file {MODEL_PATH} not found!")
        return
    
    status_text.text(f"Initializing Anonymizer (AI={'OFF' if manual_only else 'ON'} + Manual Overrides)...")
    
    # Open Video Clip
    clip = VideoFileClip(input_path)
    
    # Use Context Manager
    with FaceAnonymizer(MODEL_PATH, manual_overrides=manual_overrides, manual_only=manual_only) as anonymizer:
        
        # Use transform() to pass time 't' (MoviePy v2 replacement for fl)
        anonymized_clip = clip.transform(lambda gf, t: anonymizer.process_frame(gf(t), t))
        
        status_text.text("Processing... Applying Pixelation.")
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

# --- Global File Loading ---
try:
    video_files = [f for f in os.listdir(CASE_DIR) if f.lower().endswith(('.mov', '.mp4'))]
    video_files.sort()
except FileNotFoundError:
    video_files = []

try:
    anon_files = [f for f in os.listdir(ANON_DIR) if f.lower().endswith('.mp4')]
    anon_files.sort()
except FileNotFoundError:
    anon_files = []
    
try:
    angio_files = [f for f in os.listdir(ANGIO_DIR) if f.lower().endswith(('.mov', '.mp4'))]
    angio_files.sort()
except FileNotFoundError:
    angio_files = []

# --- App UI ---

st.title("🏥 STANDUP II: Case Video Studio")

tab1, tab2, tab3 = st.tabs(["🕵️ Face Anonymizer", "🎬 Sequence Editor", "🛠️ Manual Fixes"])

# --- TAB 1: Anonymizer ---
with tab1:
    st.header("Anonymize Patient Videos")
    st.markdown("Select videos from the `case/` folder to pixelate faces.")
    
    if not os.path.exists(MODEL_PATH):
        st.error(f"⚠️ Model file `{MODEL_PATH}` not found in directory. Please check installation.")

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
        if anon_files:
            st.write(anon_files)
        else:
            st.write("No files yet.")
        
    with col_b:
        st.markdown("**Angiogram Clips**")
        if angio_files:
            st.write(angio_files)
        else:
            st.write("No files found.")

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
        
        # Exclude special files that go to the end
        special_end_files = ['cta.mov', 'mri.mov']
        regular_angio = [f for f in angio_files if f not in special_end_files]
        
        current_idx = 0
        for i, count in enumerate(chunk_sizes):
            date_val = angio_dates[i]
            for _ in range(count):
                if current_idx < len(regular_angio):
                    f = regular_angio[current_idx]
                    path = os.path.join(ANGIO_DIR, f)
                    
                    if is_file_ready(path):
                        angio_items.append({
                            "file": f,
                            "path": path,
                            "type": "Angiogram",
                            "date": date_val
                        })
                    current_idx += 1

        # Add Special Files at the END (Future Date)
        for f in special_end_files:
             if f in angio_files:
                path = os.path.join(ANGIO_DIR, f)
                if is_file_ready(path):
                    angio_items.append({
                        "file": f,
                        "path": path,
                        "type": "Angiogram (End)",
                        "date": datetime.datetime(2030, 1, 1) # Force End
                    })

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
                # Use MoviePy to load (Disable audio to prevent sync bugs, Standardize FPS)
                clip = VideoFileClip(item["path"], audio=False)
                
                # Standardize Frame Rate to 30 FPS (Fixes 120fps vs 30fps concatenation issues)
                clip = clip.with_fps(30)
                
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
                audio=False,
                fps=30,
                ffmpeg_params=['-pix_fmt', 'yuv420p']
            )
            
            prog.progress(100)
            stat.success("Rendering Complete!")
            st.video(output_path_final)
            
        except Exception as e:
            st.error(f"Render Error: {e}")



# --- TAB 3: Manual Fixes ---
with tab3:
    st.header("Manual Correction Tool")
    st.markdown("Use this to **force-blur** a specific area (like a face missed by AI) for a time range.")
    
    if not video_files:
        st.warning("No videos found.")
    else:
        fix_video_sel = st.selectbox("Select Source Video to Fix", video_files, key="fix_sel")
        fix_video_path = os.path.join(CASE_DIR, fix_video_sel)
        
        # Check for existing anonymized version to review
        anon_filename = f"anon_{fix_video_sel.rsplit('.', 1)[0]}.mp4"
        anon_path = os.path.join(ANON_DIR, anon_filename)
        
        if os.path.exists(anon_path):
            st.markdown("### 🔎 Step 1: Review Current Anonymization")
            st.caption("Watch this to identify timestamps where faces are visible.")
            st.video(anon_path)
        else:
            st.warning("No anonymized version found yet. Process it in Tab 1 first, or use this tool to set up preemptive fixes.")

        st.divider()
        st.markdown("### 🛠️ Step 2: Add Manual Blurs")
        st.caption("Locate the frame in the **Source Video** and draw a box.")
        
        # Load clip info (lightweight)
        try:
            with VideoFileClip(fix_video_path) as clip:
                duration = clip.duration
                W, H = clip.w, clip.h
        except:
             st.error("Could not load video metadata.")
             duration = 10.0
             W, H = 1920, 1080
        
        c_preview, c_controls = st.columns([3, 2])
        
        with c_controls:
            st.subheader("1. Locate & Define Box")
            t_preview = st.slider("Preview Time (s)", 0.0, duration, 0.0, 0.1)
            
            st.write(" **Box Position**")
            col_x, col_y = st.columns(2)
            box_x = col_x.number_input("X (Left)", 0, W, W//2 - 100, step=10)
            box_y = col_y.number_input("Y (Top)", 0, H, H//5, step=10)
            
            st.write(" **Box Size**")
            col_w, col_h = st.columns(2)
            box_w = col_w.number_input("Width", 10, W, 200, step=10)
            box_h = col_h.number_input("Height", 10, H, 200, step=10)
            
            st.subheader("2. Set Time Range")
            range_col1, range_col2 = st.columns(2)
            start_t = range_col1.number_input("Start Time (s)", 0.0, duration, t_preview, 0.1)
            end_t = range_col2.number_input("End Time (s)", 0.0, duration, min(t_preview+2.0, duration), 0.1)

            if st.button("Add Manual Fix"):
                if end_t > start_t:
                    if "manual_fixes" not in st.session_state:
                        st.session_state.manual_fixes = {}
                    if fix_video_sel not in st.session_state.manual_fixes:
                        st.session_state.manual_fixes[fix_video_sel] = []
                    
                    # Store 6-tuple: (Start, End, X, Y, W, H)
                    st.session_state.manual_fixes[fix_video_sel].append((start_t, end_t, box_x, box_y, box_w, box_h))
                    st.success(f"Added fix: {start_t}s - {end_t}s")
                else:
                    st.error("End time must be > Start time")

        with c_preview:
            st.subheader("Preview")
            # Get frame
            try:
                with VideoFileClip(fix_video_path) as clip:
                    frame = clip.get_frame(t_preview)
                    # Draw Box
                    frame_copy = frame.copy()
                    cv2.rectangle(frame_copy, (int(box_x), int(box_y)), (int(box_x+box_w), int(box_y+box_h)), (255, 0, 0), 3)
                    st.image(frame_copy, caption=f"Frame at {t_preview}s", use_column_width=True)
            except Exception as e:
                st.error(f"Error reading frame: {e}")

        st.divider()
        
        # Show Current Fixes
        if "manual_fixes" in st.session_state and fix_video_sel in st.session_state.manual_fixes:
            fixes = st.session_state.manual_fixes[fix_video_sel]
            if fixes:
                st.write(f"**active Fixes for {fix_video_sel}:**")
                for i, fix in enumerate(fixes):
                    c_fix_info, c_fix_del = st.columns([4, 1])
                    with c_fix_info:
                        if len(fix) == 6:
                            s, e, x, y, w, h = fix
                            st.info(f"#{i+1}:  ⏱️ {s:.1f}-{e:.1f}s  |  📦 {x},{y} ({w}x{h})")
                        else:
                            s, e = fix
                            st.info(f"#{i+1}:  ⏱️ {s:.1f}-{e:.1f}s  |  [Legacy Default Box]")
                    with c_fix_del:
                        if st.button("🗑️", key=f"del_fix_{i}"):
                            st.session_state.manual_fixes[fix_video_sel].pop(i)
                            st.rerun()
                
                c_act1, c_act2 = st.columns(2)
                with c_act1:
                    if st.button("Clear All Fixes"):
                        st.session_state.manual_fixes[fix_video_sel] = []
                        st.rerun()
                with c_act2:
                    manual_only_mode = st.checkbox("Manual Only (Disable Auto-AI)", value=False, help="If checked, ONLY your manual boxes will be blurred. The AI face detector will be turned off.")
                    
                    if st.button("⚡ Apply Fixes & Re-Process", type="primary"):
                        o_filename = f"anon_{fix_video_sel.rsplit('.', 1)[0]}.mp4"
                        o_path = os.path.join(ANON_DIR, o_filename)
                        
                        p_bar = st.progress(0)
                        stat_txt = st.empty()
                        
                        try:
                            process_video_anonymization(
                                fix_video_path, 
                                o_path, 
                                p_bar, 
                                stat_txt, 
                                manual_overrides=fixes,
                                manual_only=manual_only_mode
                            )
                            st.success(f"Fixed video saved: {o_filename}")
                            st.video(o_path)
                        except Exception as e:
                            st.error(f"Error: {e}")
