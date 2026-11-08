import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core";

export default function AudioChannel() {
    const [channels, setChannels] = useState([]);
    const [selectedChannel, setSelectedChannel] = useState('');
    const [text, setText] = useState('');
    const [loading, setLoading] = useState(false);

    function changeChannel(event) {
        setSelectedChannel(event.target.value);
    }

    useEffect(() => {
        async function loadChannels() {
            // Once you are ready, you can replace this with a Rust invoke 
            // that fetches real virtual device names!
            setChannels(['VirtualSpeaker', '2', '3']);
        }
        loadChannels();
    }, []); 

    async function speakText(event) {
        event.preventDefault(); // Keeps the form from reloading the page
        if (!text.trim()) return;
        setLoading(true);

        try {
            // Simply pass the text across the bridge to your local TTS handler
            await invoke("speak_local_tts", { 
                text: text, 
                channelId: selectedChannel 
            });
        } catch (err) {
            console.error("Local speech injection failed:", err);
        } finally {
            setLoading(false);
        }
    }

    return (
        <form onSubmit={speakText} style={{ padding: "20px", fontFamily: "sans-serif" }}>
            <div className="dropdown">
                <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                    Microphone Channel
                </label>
                <select name="micChannel" value={selectedChannel} onChange={changeChannel} required style={{ padding: "5px" }}>
                    <option value="">Select Preferred Mic</option>
                    {channels.map(channel => (
                        <option key={channel} value={channel}>{channel}</option>
                    ))}
                </select>
            </div>
            
            <div style={{ marginTop: "15px" }}>
                <textarea 
                    value={text} 
                    onChange={(e) => setText(e.target.value)}
                    placeholder="Type what you want your microphone to say..."
                    style={{ width: "100%", height: "80px", padding: "8px", boxSizing: "border-box" }}
                />
                <div style={{ marginTop: "10px" }}>
                    <button type="submit" disabled={loading} style={{ padding: "8px 16px", cursor: "pointer" }}>
                        {loading ? "Injecting Speech..." : "Speak to Mic"}
                    </button>
                </div>
            </div>
        </form>
    );
}