import { addRestaurant } from "@/util"
import { useState } from "react"
import { Restaurant } from "@/model"

export default function Component () {
    const [name, setName] = useState('');
    const [address, setAddress] = useState('');
    const [email, setEmail] = useState('');
    const [description, setDescription] = useState('');
    const [rating, setRating] = useState('');
    const [price, setPrice] = useState('');

    const handleSubmit = async (e:any) => {
        e.preventDefault();
        const restaurant = { name, address, email, description, rating, price };
        const res = await addRestaurant({ ...restaurant});
        const json = await res.json()
        if (!res.ok) throw Error(json.message)
        console.log('Restaurant created!')
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen py-2">
        <form onSubmit={handleSubmit}>
            <label>
                Name:
                <input type="text" value={name} onChange={(e) => setName(e.target.value)} />
            </label>
            <label>
                Address:
                <input type="text" value={address} onChange={(e) => setAddress(e.target.value)} />
            </label>
            <label>
                Email:
                <input type="text" value={email} onChange={(e) => setEmail(e.target.value)} />
            </label>
            <label>
                Description:
                <input type="text" value={description} onChange={(e) => setDescription(e.target.value)} />
            </label>
            <label>
                Rating:
                <input type="text" value={rating} onChange={(e) => setRating(e.target.value)} />
            </label>
            <label>
                Price:
                <input type="text" value={price} onChange={(e) => setPrice(e.target.value)} />
            </label>
            <button>Submit</button>
        </form>
        </div>
    )
}