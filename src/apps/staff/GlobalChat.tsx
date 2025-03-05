import React, { useState, useEffect, useRef } from 'react';
import {invoke} from "@tauri-apps/api/core";
import {useStaffAuth} from "@/context/StaffAuthProvider.tsx";

interface Message {
    text: string;
    sender: string;
    timestamp: number;
}

const GlobalChat: React.FC = () => {
    const {username} = useStaffAuth();
    const [messages, setMessages] = useState<Message[]>([]);
    const [newMessage, setNewMessage] = useState('');
    const pollingRef = useRef<NodeJS.Timeout | null>(null);

    useEffect(() => {
        const fetchInitialMessages = async () => {
            try {
                const initialMessages = await invoke<Message[]>('fetch_new_messages');
                setMessages(initialMessages);
            } catch (error) {
                console.error('Failed to fetch messages:', error);
            }
        };

        const startPolling = () => {
            pollingRef.current = setInterval(async () => {
                try {
                    const newMessages = await invoke<Message[]>('fetch_new_messages');

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
                sender: username
            });

            setNewMessage('');

            const newMessages = await invoke<Message[]>('fetch_new_messages');
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
        <div className="chat-container">
            <div className="messages">
                {messages.map((msg, index) => (
                    <div key={index} className="message">
                        <strong>{msg.sender}: </strong>
                        {msg.text}
                    </div>
                ))}
            </div>
            <div className="message-input">
                <input
                    type="text"
                    value={newMessage}
                    onChange={(e) => setNewMessage(e.target.value)}
                    onKeyPress={(e) => e.key === 'Enter' && sendMessage()}
                />
                <button onClick={sendMessage}>Send</button>
            </div>
        </div>
    );
};

export default GlobalChat;