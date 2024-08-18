// TODO: Implement struct 'Order', type 'OrderFilter' and function 'filter_orders' as described in the description.

#[cfg(test)]
mod tests {
    use crate::{filter_orders, Order, OrderFilter};

    #[test]
    fn test_filter_order_price_greater_than_100() {
        let orders = vec![
            Order {
                id: 1,
                customer: String::from("Alice"),
                price: 150.0,
            },
            Order {
                id: 2,
                customer: String::from("Bob"),
                price: 75.0,
            },
            Order {
                id: 3,
                customer: String::from("Alice"),
                price: 200.0,
            },
        ];

        let price_filter: OrderFilter = Box::new(|order| order.price >= 100.0);
        let ret = filter_orders(orders, vec![price_filter]);

        assert_eq!(ret.len(), 2);
    }

    #[test]
    fn test_filter_order_customer_is_alice() {
        let orders = vec![
            Order {
                id: 1,
                customer: String::from("Alice"),
                price: 150.0,
            },
            Order {
                id: 2,
                customer: String::from("Bob"),
                price: 75.0,
            },
            Order {
                id: 3,
                customer: String::from("Alice"),
                price: 200.0,
            },
        ];

        let customer_filter: OrderFilter = Box::new(|order| order.customer == "Alice");
        let ret = filter_orders(orders, vec![customer_filter]);

        assert_eq!(ret.len(), 2);
    }

    #[test]
    fn test_filter_order_price_greater_than_200_and_customer_is_alice() {
        let orders = vec![
            Order {
                id: 1,
                customer: String::from("Alice"),
                price: 150.0,
            },
            Order {
                id: 2,
                customer: String::from("Bob"),
                price: 75.0,
            },
            Order {
                id: 3,
                customer: String::from("Alice"),
                price: 200.0,
            },
        ];

        let price_filter: OrderFilter = Box::new(|order| order.price >= 200.0);
        let customer_filter: OrderFilter = Box::new(|order| order.customer == "Alice");
        let ret = filter_orders(orders, vec![price_filter, customer_filter]);

        assert_eq!(ret.len(), 1);
    }
}
