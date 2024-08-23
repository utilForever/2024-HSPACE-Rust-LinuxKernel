#[allow(dead_code)]
struct Order {
    id: u32,
    customer: String,
    price: f64,
}

#[allow(dead_code)]
type OrderFilter = Box<dyn Fn(&Order) -> bool>;

#[allow(dead_code)]
fn filter_orders(orders: Vec<Order>, filters: Vec<OrderFilter>) -> Vec<Order> {
    orders
        .into_iter()
        .filter(|order| filters.iter().all(|filter| filter(order)))
        .collect()
}

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
