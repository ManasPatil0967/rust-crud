import { NextApiRequest, NextApiResponse } from 'next';
import { addUser, addRestaurant, addReview, getUsers, getRestaurants, getReviews } from '@/util';
import { User, Restaurant, Review } from '@/model';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  // Add User
  if (req.method === 'POST' && req.url === '/add_user') {
    const user: User = req.body;
    const response = await addUser(user);
    return res.status(200).json(response);
  }

  // Add Restaurant
  if (req.method === 'POST' && req.url === '/add_restaurant') {
    const restaurant: Restaurant = req.body;
    const response = await addRestaurant(restaurant);
    return res.status(200).json(response);
  }
  
  // Add Review
  if (req.method === 'POST' && req.url === '/add_review') {
    const review: Review = req.body;
    const response = await addReview(review);
    return res.status(200).json(response);
  }

  // Get Users
  if (req && req.url && req.method === 'GET' && req.url.startsWith('/get_users/')) {
    const username = req.url.split('/')[2];
    const response = await getUsers(username);
    return res.status(200).json(response);
  }

  // Get Restaurants
  if (req && req.url && req.method === 'GET' && req.url.startsWith('/get_restaurants/')) {
    const name = req.url.split('/')[2];
    const response = await getRestaurants(name);
    return res.status(200).json(response);
  }

  // Get Reviews
  if (req && req.url && req.method === 'GET' && req.url.startsWith('/get_reviews/')) {
    const [restaurant, username] = req.url.split('/')[2].split('&');
    const response = await getReviews(restaurant, username);
    return res.status(200).json(response);
  }

  // 404 for all other cases
  return res.status(404).end();
}
