import React from 'react';
import { useState } from 'react';
import { addUser } from '../util/index'

export default function Component (){
    const [username, setUserName] = useState('');
    const [password, setPassword] = useState('');
    const [restaurants, setRestaurants] = useState('');
    
    const handleSubmit = async (e:any) => {
        e.preventDefault();
        const user = { username, password, restaurants };
        const res = await addUser({ ...user, restaurants: [restaurants] });
        const json = await res.json()
        if (!res.ok) throw Error(json.message)
        console.log('User created!')
    }
    return (
        <div className="flex flex-col items-center justify-center min-h-screen py-2">
        <form onSubmit={handleSubmit}>
            <label>
                Name:
                <input type="text" value={username} onChange={(e) => setUserName(e.target.value)} />
            </label>
            <label>
                Password:
                <input type="text" value={password} onChange={(e) => setPassword(e.target.value)} />
            </label>
            <label>
                Restaurants:
                <input type="text" value={restaurants} onChange={(e) => setRestaurants(e.target.value)} />
            </label>
            <button>Submit</button>
        </form>
        </div>
    )
}