import React, { useState, useEffect, useRef } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { useStaffAuth } from "@/context/StaffAuthProvider.tsx";

interface Message {
    text: string;
    sender: string;
    timestamp: number;
}

const GlobalChat: React.FC = () => {
    const { username } = useStaffAuth();
    const [messages, setMessages] = useState<Message[]>([]);
    const [newMessage, setNewMessage] = useState('');
    const pollingRef = useRef<NodeJS.Timeout | null>(null);

    useEffect(() => {
        const fetchInitialMessages = async () => {
            try {
                const initialMessages = await invoke<Message[]>('fetch_new_messages', { group: "Marketing" });
                setMessages(initialMessages);
            } catch (error) {
                console.error('Failed to fetch messages:', error);
            }
        };

        const startPolling = () => {
            pollingRef.current = setInterval(async () => {
                try {
                    const newMessages = await invoke<Message[]>('fetch_new_messages', { group: "Marketing" });
                    if (newMessages.length > 0) {
                        setMessages(prevMessages => {
                            const combinedMessages = [...prevMessages, ...newMessages];
                            const uniqueMessages = Array.from(
                                new Map(combinedMessages.map(m => [m.timestamp, m])).values()
                            );
                            return uniqueMessages.sort((a, b) => a.timestamp - b.timestamp);
                        });
                    }
                } catch (error) {
                    console.error('Error polling for messages:', error);
                }
            }, 3000);
        };

        fetchInitialMessages();
        startPolling();

        return () => {
            if (pollingRef.current) {
                clearInterval(pollingRef.current);
            }
        };
    }, []);

    const sendMessage = async () => {
        if (!newMessage.trim()) return;
        try {
            await invoke('send_chat_message', {
                text: newMessage,
                sender: username,
                group: "Marketing"
            });
            setNewMessage('');

            const newMessages = await invoke<Message[]>('fetch_new_messages', { group: "Marketing" });
            setMessages(prevMessages => {
                const combinedMessages = [...prevMessages, ...newMessages];
                const uniqueMessages = Array.from(
                    new Map(combinedMessages.map(m => [m.timestamp, m])).values()
                );
                return uniqueMessages.sort((a, b) => a.timestamp - b.timestamp);
            });
        } catch (error) {
            console.error('Failed to send message:', error);
        }
    };

    return (
        <div className="flex flex-col h-screen bg-gray-900 text-white p-4">
            <div className="flex-1 overflow-y-auto p-4 space-y-3 border border-gray-700 rounded-lg bg-gray-800">
                {messages.map((msg, index) => (
                    <div
                        key={index}
                        className={`p-2 rounded-lg w-fit max-w-xs ${msg.sender === username ? 'bg-purple-500 ml-auto' : 'bg-white text-black'}`}
                    >
                        <span className={`font-bold ${msg.sender === username ? 'text-white' : 'text-purple-500'}`}>
                            {msg.sender}
                        </span>
                        <p>{msg.text}</p>
                    </div>
                ))}
            </div>

            <div className="flex items-center mt-4 space-x-2">
                <input
                    type="text"
                    className="flex-1 p-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                    placeholder="Type your message..."
                    value={newMessage}
                    onChange={(e) => setNewMessage(e.target.value)}
                />
                <button
                    className="px-4 py-2 bg-purple-500 rounded-lg hover:bg-purple-600"
                    onClick={sendMessage}
                >
                    Send
                </button>
            </div>
        </div>
    );
};

export default GlobalChat;