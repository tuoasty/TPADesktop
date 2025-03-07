import React, { useState, useEffect, useRef } from 'react';
import { invoke } from "@tauri-apps/api/core";
import {useCustomerAuth} from "@/context/CustomerAuthProvider.tsx";

interface Message {
    text: string;
    sender: string;
    timestamp: number;
}

const CustomerChatService: React.FC = () => {
    const { customerId, name } = useCustomerAuth()
    const [messages, setMessages] = useState<Message[]>([]);
    const [newMessage, setNewMessage] = useState('');
    const pollingRef = useRef<NodeJS.Timeout | null>(null);

    useEffect(() => {
        const fetchInitialMessages = async () => {
            try {
                const initialMessages = await invoke<Message[]>('fetch_new_messages', { group: `CustomerService-${customerId}` });
                setMessages(initialMessages);
            } catch (error) {
                console.error('Failed to fetch messages:', error);
            }
        };

        const startPolling = () => {
            pollingRef.current = setInterval(async () => {
                try {
                    const newMessages = await invoke<Message[]>('fetch_new_messages', { group:`CustomerService-${customerId}` });
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
                sender: name,
                group: `CustomerService-${customerId}`
            });
            setNewMessage('');

            const newMessages = await invoke<Message[]>('fetch_new_messages', { group: `CustomerService-${customerId}` });
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
                        className={`p-2 rounded-lg w-fit max-w-xs ${msg.sender === name ? 'bg-blue-500 ml-auto' : 'bg-white text-black'}`}
                    >
                        <span className={`font-bold ${msg.sender === name ? 'text-white' : 'text-blue-500'}`}>
                            {msg.sender}
                        </span>
                        <p>{msg.text}</p>
                    </div>
                ))}
            </div>

            <div className="flex items-center mt-4 space-x-2">
                <input
                    type="text"
                    className="flex-1 p-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Type your message..."
                    value={newMessage}
                    onChange={(e) => setNewMessage(e.target.value)}
                />
                <button
                    className="px-4 py-2 bg-blue-500 rounded-lg hover:bg-blue-600"
                    onClick={sendMessage}
                >
                    Send
                </button>
            </div>
        </div>
    );
};

export default CustomerChatService;