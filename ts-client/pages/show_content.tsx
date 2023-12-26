import { getUsers, getRestaurants, getReviews } from "@/util"
import { useState } from "react"
import { User, Restaurant, Review } from "@/model"

export default function Component () {
    const [username, setUserName] = useState('');
    const [restaurantName, setRestaurantName] = useState('');
    const [user, setUser] = useState<User>();
    const [restaurant, setRestaurant] = useState<Restaurant>();
    const [review, setReview] = useState<Review>();

    const getUserFromUsername = async (username: string) => {
        const res = await getUsers(username);
        const user: User = res;
        setUser(user);
        return user;
    }

    const getRestaurantFromName = async (name: string) => {
        const res = await getRestaurants(name);
        const json = await res.json();
        if (!res.ok) throw Error(json.message);
        const restaurant: Restaurant = json.data;
        setRestaurant(restaurant);
        return restaurant;
    }

    const getReviewFromReview = async (reviewName: string) => {
        const res = await getReviews(username, restaurantName);
        const json = await res.json();
        if (!res.ok) throw Error(json.message);
        const review: Review = json.data;
        setReview(review);
        return review;
    }

    const handleSubmit = async (e:any) => {
        e.preventDefault();
        const user = await getUserFromUsername(username);
        const restaurant = await getRestaurantFromName(restaurantName);
        const review = await getReviewFromReview(username);
        console.log(user);
        console.log(restaurant);
        console.log(review);
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen py-2">
        <form onSubmit={handleSubmit}>
            <label>
                User:
                <input type="text" value={username} onChange={(e) => setUserName(e.target.value)} />
            </label>
            <label>
                Restaurant:
                <input type="text" value={restaurantName} onChange={(e) => setRestaurantName(e.target.value)} />
            </label>
            <button>Submit</button>
        </form>
        <p>{
            user?.username ?? ''
        }</p>
        <p>{
            restaurant?.name ?? ''
        }</p>
        <p>{
            review?.comment ?? ''
        }</p>
        </div>
    )
}