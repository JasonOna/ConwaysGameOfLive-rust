use grrs;

#[test]
fn test_set_prev_state() {
    let mut a = grrs::Cell {
      x: 0,
      y: 1,
      currently_alive: true,
      previously_alive: false,
    };

    a.set_prev_state();

    assert_eq!(a.previously_alive, true);
}

#[test]
fn test_set_currently_alive() {
    let mut a = grrs::Cell::new(1, 2, false);

    a.set_currently_alive(true);

    assert_eq!(a.currently_alive, true);
}