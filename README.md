# rust-timezones-chrono-tz

An example of working with timezones in Rust using chrono-tz

Run with `cargo run` and then you can:

Add date times with:

```bash
curl -X POST http://localhost:8080/create -d '{"date_time": "1996-12-19T16:39:57+02:00"}' -H "content-type: application/json"
```

And fetch them, converted to a given timezone with:

```bash
// UTC
curl http://localhost:8080/fetch/UTC
["1996-12-19T15:39:57+01:00"]

// UTC +01:00
curl http://localhost:8080/fetch/Africa%2FAlgiers
["1996-12-19T14:39:57+00:00"]
```

Valid time zone strings can be found [here](https://docs.rs/chrono-tz/0.5.3/chrono_tz/enum.Tz.html).
