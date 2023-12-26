export type User = {
    username: string,
    password: string,
    restaurants: string[],
}

export type Restaurant = {
    name: string,
    address: string,
    email: string,
    description: string,
    rating: string,
    price: string,
}

export type Review = {
    restaurant: string,
    user: string,
    rating: string,
    comment: string,
}