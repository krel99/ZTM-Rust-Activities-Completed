// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct CheckIn;
struct OnLoading;
struct Offloading;
struct AwaitingPickup;
struct EndCustody;

#[allow(dead_code)]
struct Luggage<State> {
    tracking_id: String,
    state: State,
}

impl Luggage<CheckIn> {
    fn new(tracking_id: impl Into<String>) -> Self {
        Luggage {
            tracking_id: tracking_id.into(),
            state: CheckIn,
        }
    }

    fn on_load(self) -> Luggage<OnLoading> {
        Luggage {
            tracking_id: self.tracking_id,
            state: OnLoading,
        }
    }
}

impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<Offloading> {
        Luggage {
            tracking_id: self.tracking_id,
            state: Offloading,
        }
    }
}

impl Luggage<Offloading> {
    fn carousel(self) -> Luggage<AwaitingPickup> {
        Luggage {
            tracking_id: self.tracking_id,
            state: AwaitingPickup,
        }
    }
}

impl Luggage<AwaitingPickup> {
    fn pick_up(self) -> Luggage<EndCustody> {
        Luggage {
            tracking_id: self.tracking_id,
            state: EndCustody,
        }
    }
}

fn main() {
    let luggage = Luggage::new("LUG-12345");
    println!("Luggage checked in: {}", luggage.tracking_id);

    let luggage = luggage.on_load();
    println!("Luggage loaded onto plane: {}", luggage.tracking_id);

    let luggage = luggage.offload();
    println!("Luggage offloaded from plane: {}", luggage.tracking_id);

    let luggage = luggage.carousel();
    println!("Luggage awaiting pickup: {}", luggage.tracking_id);

    let luggage = luggage.pick_up();
    println!("Luggage picked up by passenger: {}", luggage.tracking_id);
}
