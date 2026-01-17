
import os
import datetime
from moviepy import VideoFileClip

CASE_DIR = "case"

def get_creation_time(filepath):
    # Try to get creation time from metadata
    try:
        # os.path.getmtime gives modification time, which might be file creation on this disk if copied
        # But for MOV files, we might need to parse atoms. 
        # Let's start with os.path.getmtime and see if it makes sense (2024/2025/2026?)
        timestamp = os.path.getmtime(filepath)
        dt = datetime.datetime.fromtimestamp(timestamp)
        return dt
    except Exception:
        return None

def analyze():
    files = [f for f in os.listdir(CASE_DIR) if f.lower().endswith(('.mov', '.mp4'))]
    files.sort()
    
    print(f"{'Filename':<20} | {'Date':<20}")
    print("-" * 45)
    
    for f in files:
        if "anonymized" in f: continue
        path = os.path.join(CASE_DIR, f)
        dt = get_creation_time(path)
        print(f"{f:<20} | {dt}")

if __name__ == "__main__":
    analyze()
