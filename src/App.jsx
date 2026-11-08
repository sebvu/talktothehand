import { useRef } from "react";
import "./App.css";

export default function App() {
  const videoRef = useRef(null);

  const allowCameraPermission = async () => {
    try {
      const userMediaStream = await navigator.mediaDevices.getUserMedia({
        video: true,
      });

      if (videoRef.current) {
        videoRef.current.srcObject = userMediaStream;
      }

      console.log(userMediaStream);
    } catch (err) {
      console.error(`ERROR CAM PERMISSIONS: ${err}`);
    }
  };

  return (
    <div data-tauri-drag-region className="app-container">
      <div className="camera">
        <video
          ref={videoRef}
          autoPlay
          className="camera__camera-display"
        ></video>
        <button
          onClick={allowCameraPermission}
          className="camera__allow-button"
        >
          Allow Camera
        </button>
      </div>
    </div>
  );
}
