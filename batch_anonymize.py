
import cv2
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
import numpy as np
import os
from moviepy import VideoFileClip

# --- Config ---
CASE_DIR = "case"
ANON_DIR = os.path.join(CASE_DIR, "anonymized")
MODEL_PATH = 'blaze_face_short_range.tflite'
os.makedirs(ANON_DIR, exist_ok=True)

class FaceAnonymizer:
    def __init__(self, model_path, confidence=0.1, pixelation_factor=20, persistence_frames=30, padding=0.3):
        self.model_path = model_path
        self.confidence = confidence
        self.pixelation_factor = pixelation_factor
        self.persistence = persistence_frames
        self.padding = padding
        self.active_faces = []

    def __enter__(self):
        base_options = python.BaseOptions(model_asset_path=self.model_path)
        options = vision.FaceDetectorOptions(base_options=base_options, min_detection_confidence=self.confidence)
        self.detector = vision.FaceDetector.create_from_options(options)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.detector.close()

    def process_frame(self, image_rgb):
        if not image_rgb.flags['WRITEABLE']:
            image_rgb = image_rgb.copy()
            
        h_img, w_img, c = image_rgb.shape
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=np.ascontiguousarray(image_rgb))
        
        try:
            detection_result = self.detector.detect(mp_image)
            current_detections = []
            if detection_result.detections:
                for detection in detection_result.detections:
                    bbox = detection.bounding_box
                    current_detections.append([bbox.origin_x, bbox.origin_y, bbox.width, bbox.height])
            
            if current_detections:
                self.active_faces = [{'bbox': box, 'ttl': self.persistence} for box in current_detections]
            else:
                for face in self.active_faces:
                    face['ttl'] -= 1
                self.active_faces = [f for f in self.active_faces if f['ttl'] > 0]
                
        except Exception as e:
            print(f"Detection Error: {e}")

        for face in self.active_faces:
            x, y, w_box, h_box = face['bbox']
            pad_w = int(w_box * self.padding)
            pad_h = int(h_box * self.padding)
            x = x - pad_w
            y = y - pad_h
            w_box = w_box + (2 * pad_w)
            h_box = h_box + (2 * pad_h)
            x = max(0, x)
            y = max(0, y)
            w_box = min(w_box, w_img - x)
            h_box = min(h_box, h_img - y)
            
            if w_box > 0 and h_box > 0:
                try:
                    roi = image_rgb[y:y+h_box, x:x+w_box]
                    small_w = max(1, w_box // self.pixelation_factor)
                    small_h = max(1, h_box // self.pixelation_factor)
                    small = cv2.resize(roi, (small_w, small_h), interpolation=cv2.INTER_LINEAR)
                    pixelated = cv2.resize(small, (w_box, h_box), interpolation=cv2.INTER_NEAREST)
                    image_rgb[y:y+h_box, x:x+w_box] = pixelated
                except Exception:
                    pass
        return image_rgb

def process_batch():
    if not os.path.exists(CASE_DIR):
        print(f"Error: Directory '{CASE_DIR}' not found.")
        return

    video_files = [f for f in os.listdir(CASE_DIR) if f.lower().endswith(('.mov', '.mp4'))]
    video_files.sort()
    
    if not video_files:
        print("No videos found to process.")
        return

    print(f"Index: Found {len(video_files)} videos. Starting batch processing...")
    print(f"Config: Pixelation={20}, Padding={0.3}, Persistence={30}")

    for i, filename in enumerate(video_files):
        input_path = os.path.join(CASE_DIR, filename)
        output_filename = f"anon_{filename.rsplit('.', 1)[0]}.mp4"
        output_path = os.path.join(ANON_DIR, output_filename)
        
        print(f"[{i+1}/{len(video_files)}] Processing {filename}...")
        
        try:
            clip = VideoFileClip(input_path)
            with FaceAnonymizer(MODEL_PATH) as anonymizer:
                anonymized_clip = clip.image_transform(anonymizer.process_frame)
                # Suppress moviepy stdout
                anonymized_clip.write_videofile(output_path, codec='libx264', audio_codec='aac', logger=None)
            print(f"Saved: {output_path}")
        except Exception as e:
            print(f"Failed to process {filename}: {e}")

    print("Batch processing complete!")

if __name__ == "__main__":
    process_batch()
