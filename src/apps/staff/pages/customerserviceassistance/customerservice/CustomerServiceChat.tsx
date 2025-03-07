import React, { useState, useEffect, useRef } from 'react';
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import {Customer} from "@/ types/customer.ts";

interface Message {
    text: string;
    sender: string;
    timestamp: number;
}

const CustomerServiceChat: React.FC = () => {
    const username = "Customer Service"
    const [messages, setMessages] = useState<Message[]>([]);
    const [newMessage, setNewMessage] = useState('');
    const [customers, setCustomers] = useState<Customer[]>([]);
    const [activeCustomer, setActiveCustomer] = useState<Customer | null>(null);
    const pollingRef = useRef<NodeJS.Timeout | null>(null);

    const fetchCustomers = async () => {
        try {
            invoke<Customer[]>("get_all_customer").then(setCustomers);
        } catch (e) {
            toast.error(`${e}`);
        }
    };

    const fetchMessages = async (customerId: number) => {
        try {
            const fetchedMessages = await invoke<Message[]>('fetch_new_messages', { group: `CustomerService-${customerId}` });
            setMessages(fetchedMessages);
        } catch (error) {
            console.error('Failed to fetch messages:', error);
        }
    };

    useEffect(() => {
        fetchCustomers();
    }, []);

    useEffect(() => {
        if (!activeCustomer) return;

        fetchMessages(activeCustomer.id);

        const startPolling = () => {
            pollingRef.current = setInterval(async () => {
                try {
                    const newMessages = await invoke<Message[]>('fetch_new_messages', { group: `CustomerService-${activeCustomer.id}` });
                    if (newMessages.length > 0) {
                        setMessages(prevMessages => {
                            const combinedMessages = [...prevMessages, ...newMessages];
                            const uniqueMessages = Array.from(new Map(combinedMessages.map(m => [m.timestamp, m])).values());
                            return uniqueMessages.sort((a, b) => a.timestamp - b.timestamp);
                        });
                    }
                } catch (error) {
                    console.error('Error polling for messages:', error);
                }
            }, 3000);
        };

        startPolling();

        return () => {
            if (pollingRef.current !== null) {
                clearInterval(pollingRef.current);
                pollingRef.current = null;
            }
        };
    }, [activeCustomer]);


    const sendMessage = async () => {
        if (!newMessage.trim() || !activeCustomer) return;
        try {
            await invoke('send_chat_message', {
                text: newMessage,
                sender: "Customer Service",
                group: `CustomerService-${activeCustomer.id}`
            });
            setNewMessage('');
            fetchMessages(activeCustomer.id);
        } catch (error) {
            console.error('Failed to send message:', error);
        }
    };

    return (
        <div className="flex h-screen bg-gray-900 text-white">
            <div className="w-1/4 bg-gray-800 p-4 border-r border-gray-700 overflow-y-auto">
                <h2 className="text-xl font-bold mb-4">Customers</h2>
                <ul>
                    {customers.map(customer => (
                        <li key={customer.id} className={`p-2 rounded-lg cursor-pointer ${activeCustomer?.id === customer.id ? 'bg-purple-500' : 'hover:bg-gray-700'}`} onClick={() => setActiveCustomer(customer)}>
                            {customer.name}
                        </li>
                    ))}
                </ul>
            </div>

            <div className="flex flex-col flex-1 p-4">
                <div className="flex-1 overflow-y-auto p-4 space-y-3 border border-gray-700 rounded-lg bg-gray-800">
                    {activeCustomer ? (
                        messages.map((msg, index) => (
                            <div key={index} className={`p-2 rounded-lg w-fit max-w-xs ${msg.sender === username ? 'bg-purple-500 ml-auto' : 'bg-white text-black'}`}>
                                <span className={`font-bold ${msg.sender === username ? 'text-white' : 'text-purple-500'}`}>{msg.sender}</span>
                                <p>{msg.text}</p>
                            </div>
                        ))
                    ) : (
                        <p className="text-gray-400">Select a customer to start chatting.</p>
                    )}
                </div>

                {activeCustomer && (
                    <div className="flex items-center mt-4 space-x-2">
                        <input
                            type="text"
                            className="flex-1 p-2 bg-gray-700 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                            placeholder="Type your message..."
                            value={newMessage}
                            onChange={(e) => setNewMessage(e.target.value)}
                        />
                        <button className="px-4 py-2 bg-purple-500 rounded-lg hover:bg-purple-600" onClick={sendMessage}>
                            Send
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
};

export default CustomerServiceChat;
