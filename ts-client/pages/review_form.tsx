import { addReview } from "@/util"
import { useState } from "react"
import { useRouter } from "next/router"
import { Review } from "@/model"

export default function Component (){
    const [user, setUser] = useState('');
    const [restaurant, setRestaurant] = useState('');
    const [rating, setRating] = useState('');
    const [comment, setComment] = useState('');
    const router = useRouter()

    const handleSubmit = async (e:any) => {
        e.preventDefault();
        const review = { restaurant, user, rating, comment };
        const res = await addReview({ ...review});
        const json = await res.json()
        if (!res.ok) throw Error(json.message)
        console.log('Review created!')
        router.push('/reviews')
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