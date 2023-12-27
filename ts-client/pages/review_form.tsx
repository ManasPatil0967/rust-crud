import { addReview } from "@/util"
import { useState } from "react"

export default function Component (){
    const [user, setUser] = useState('');
    const [restaurant, setRestaurant] = useState('');
    const [rating, setRating] = useState('');
    const [comment, setComment] = useState('');

    const handleSubmit = async (e:any) => {
        e.preventDefault();
        const review = { restaurant, user, rating, comment };
        const res = await addReview({ ...review});
        console.log(res);
        console.log('Review created!')
    }
    return (
        <div className="flex flex-col items-center justify-center min-h-screen py-2">
        <form onSubmit={handleSubmit}>
            <label>
                User:
                <input type="text" value={user} onChange={(e) => setUser(e.target.value)} />
            </label>
            <label>
                Restaurant:
                <input type="text" value={restaurant} onChange={(e) => setRestaurant(e.target.value)} />
            </label>
            <label>
                Rating:
                <input type="text" value={rating} onChange={(e) => setRating(e.target.value)} />
            </label>
            <label>
                Comment:
                <input type="text" value={comment} onChange={(e) => setComment(e.target.value)} />
            </label>
            <button>Submit</button>
        </form>
        </div>
    )
    }
