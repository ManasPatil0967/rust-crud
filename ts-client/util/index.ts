import { User, Restaurant, Review } from "@/model";

async function apiRequest<T>(url: string, method: string, body?: T) {
    const response = await fetch(url, {
      method,
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });
  
    if (!response.ok) {
      throw new Error(`API request failed with status ${response.status}`);
    }
  
    return response.json();
  }

  export function addUser(user: User) {
    return apiRequest<User>('http://127.0.0.1:5000/add_user', 'POST', user);
  }
  
  export function addRestaurant(restaurant: Restaurant) {
    return apiRequest<Restaurant>('http://127.0.0.1:5000/add_restaurant', 'POST', restaurant);
  }
  
  export function addReview(review: Review) {
    return apiRequest<Review>('http://127.0.0.1:5000/add_review', 'POST', review);
  }
  
  export function getUsers(username: string) {
    return apiRequest<User>(`http://127.0.0.1:5000/get_users/${username}`, 'GET');
  }
  
  export function getRestaurants(name: string) {
    return apiRequest<Restaurant>(`http://127.0.0.1:5000/get_restaurants/${name}`, 'GET');
  }
  
  export function getReviews(restaurant: string, username: string) {
    return apiRequest<Review>(`http://127.0.0.1:5000/get_reviews/${restaurant}&${username}`, 'GET');
  }