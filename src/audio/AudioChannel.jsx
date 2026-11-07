import { useEffect, useState } from "react"

function AudioChannel() {
    const [channels, setChannels] = useState([]);
    const [selectedChannel, setSelectedChannel] = useState('');

    function changeChannel(event) {
        const { name, value } = event.target;

        setSelectedChannel(value);
    }

    useEffect(() => {
        async function loadChannels() {
            setChannels(['1', '2', '3'])
        }

        loadChannels();
    })

    return (
        <div className="dropdown">
            <label>Microphone channel</label>
            <select name="micChannel" value={selectedChannel} onChange={changeChannel} required>
                <option>Select Preferred Mic</option>
                {channels.map(channel => (
                    <option key={channel} value={channel} onChange={changeChannel}>{channel}</option>
                ))}
            </select>
        </div>
    )
}

export default AudioChannel;