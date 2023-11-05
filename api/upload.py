from fastapi import FastAPI, File, UploadFile, StreamingResponse
from typing import List
import subprocess
import base64
import glob
import os

app = FastAPI()

@app.post("/api/upload", response_class=StreamingResponse, media_type="text/html")
async def upload_files(files: List[UploadFile]):
    # Store uploaded files temporarily
    uploaded_images = []
    for uploaded_file in files:
        with open(uploaded_file.filename, "wb") as f:
            f.write(uploaded_file.file.read())
        uploaded_images.append(uploaded_file.filename)

    # Process uploaded files using the Rust binary
    for image_path in uploaded_images:
        rust_binary = "/bin/jpeg_extractor"
        subprocess.run(["mkdir", image_path])
        subprocess.run(["cd", image_path])
        subprocess.run([rust_binary, image_path])
        subprocess.run(["cd", ".."])

    def generate_html():
        yield b"<html><body>"

        # Create a list of base64-encoded images from img-*.jpg files
        for image_file in glob.glob("*/img-*.jpg"):
            with open(image_file, "rb") as f:
                image_data = f.read()
                base64_image = base64.b64encode(image_data).decode("utf-8")
                file_name = os.path.basename(os.path.dirname(image_file))  # Extract the associated file name
                img_tag = f'<p>File: {file_name}</p><img src="data:image/jpeg;base64,{base64_image}">'
                yield img_tag.encode("utf-8")
            os.remove(image_file)  # Delete processed images

        yield b"</body></html>"

    return generate_html()