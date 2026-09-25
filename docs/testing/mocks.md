# Mock Data, Traffic & Fuzzing

MontRS provides deterministic testing primitives for synthetic data generation,
in-process traffic stress, and property-based fuzzing.

---

## 1. Multi-cultural mock data (`montrs-test` feature `mock`)

`MockData` generates realistic, culturally diverse test values from a seeded
`TestRng`:

```rust
use montrs_test::prelude::*;

let data = MockData::seeded(42);

println!("Name:    {}", data.name());
println!("Email:   {}", data.email());
println!("UUID:    {}", data.uuid());
println!("Address: {}", data.address());
println!("Date:    {}", data.date_iso());
```

### Supported regions and naming conventions

- **Islamic / Arabic**: Muhammad, Fatima, Omar, Aisha, Tariq, Zainab, Al-Mansoor, Al-Ghamdi, Al-Otaibi, Barghouthi, Darwish, Sharif, Bashir, Khan
- **Palestinian & Saudi**: Al-Quds, Ramallah, Nablus, Hebron, Riyadh, Jeddah, Makkah, Madinah
- **Nigerian** (Hausa, Yoruba, Igbo): Chinedu, Ngozi, Olumide, Babajide, Emeka, Folake, Adeyemi, Okonkwo, Balogun, Danjuma, Bello
- **Chinese**: Wei, Fang, Lei, Ming, Ting, Jun, Wang, Li, Zhang, Liu, Chen, Beijing, Shanghai, Guangzhou
- **Indonesian**: Budi, Siti, Agus, Dewi, Bambang, Sri, Kusuma, Pratama, Wijaya, Jakarta, Surabaya, Bandung
- **French**: Jean, Marie, Pierre, Camille, Antoine, Clara, Dubois, Martin, Bernard, Paris, Lyon, Marseille
- **Spanish**: Carlos, Sofia, Mateo, Lucia, Diego, Elena, Garcia, Rodriguez, Gonzalez, Madrid, Barcelona, Valencia

Because it is driven by `TestRng`, the same seed generates the **same sequence
on every run and on every platform**.

---

## 2. In-process synthetic traffic (`montrs-test` feature `traffic`)

`Traffic` drives a `TestClient` or raw request closure with N synthetic requests
**without opening any network sockets**:

```rust
use montrs_test::prelude::*;

let traffic = Traffic::profile(TrafficProfile::Smoke); // 50 requests
let report = traffic.run_client(&client, |i| format!("/users/{}", i));

// Assert 0 failures and P95 latency ceiling
report.assert_ok(std::time::Duration::from_millis(50));

println!("p50: {:?}", report.p50());
println!("p95: {:?}", report.p95());
println!("p99: {:?}", report.p99());
println!("req/s: {:.2}", report.reqs_per_sec());
```

### Profiles

- `TrafficProfile::Smoke` — 50 requests
- `TrafficProfile::Spike` — 200 rapid requests
- `TrafficProfile::Soak` — 1,000 requests

---

## 3. Property fuzzing (`montrs-test` feature `fuzz`)

Driven by `proptest`:

```rust
use montrs_test::fuzz::*;

#[test]
fn fuzz_user_id_parsing() {
    let res = run_fuzz(any::<i32>(), 200, |id| {
        // Run input through a loader/action
        prop_assert!(id.is_finite());
        Ok(())
    });
    assert!(res.is_ok());
}
```
