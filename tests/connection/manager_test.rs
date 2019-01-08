extern crate phantomchain_client;

use phantomchain_client::{Connection, Manager};

#[test]
fn test_create_connection() {
    let conn = Connection::new("test");
    let mut manager = Manager::new();

    assert!(manager.connect(&conn).is_ok());
    assert_eq!(manager.connections().count(), 1);
}

#[test]
fn test_create_existing_connection() {
    let conn1 = Connection::new("test1");
    let conn2 = Connection::new("test2");

    let mut manager = Manager::new();
    assert!(manager.connect(&conn1).is_ok());
    assert!(manager.connect(&conn2).is_err());
}

#[test]
fn test_remove_connection() {
    let conn = Connection::new("test1");
    let mut manager = Manager::new();

    assert!(manager.connect(&conn).is_ok());
    manager.disconnect("");
    assert_eq!(manager.connections().count(), 0);
}

#[test]
fn test_get_connection() {
    let conn = Connection::new("test1");
    let mut manager = Manager::new();

    assert!(manager.connect(&conn).is_ok());
    let default_conn = manager.connection();
    assert!(default_conn.is_some());
}

#[test]
fn test_get_non_existing_connection() {
    let manager = Manager::new();
    let default_conn = manager.connection();
    assert!(default_conn.is_none());
}

#[test]
fn test_get_default_connection() {
    let manager = Manager::new();
    let default = manager.get_default_connection();
    assert_eq!(default, "main");
}

#[test]
fn test_set_default_connection() {
    let mut manager = Manager::new();
    manager.set_default_connection("test");

    let default = manager.get_default_connection();
    assert_eq!(default, "test");
}

#[test]
fn test_get_all_connections() {
    let conn1 = Connection::new("test1");
    let conn2 = Connection::new("test3");
    let mut manager = Manager::new();

    assert!(manager.connect_as(&conn1, "test1").is_ok());
    assert!(manager.connect_as(&conn2, "test3").is_ok());

    let connections = manager.connections();
    assert_eq!(connections.count(), 2);
}
