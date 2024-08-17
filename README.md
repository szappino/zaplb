# ZapLB
---
Basic L4 load balancer written with the purpose to learn [Rust](https://www.rust-lang.org/)

### Install
---
```cargo install --git https://github.com/szappino/zaplb.git --bin zapLB```

copy in a directory the configuration file:
``` JSON
{
  "address": "127.0.0.1",
  "port" : 8000,
  "targets" : [
    {
      "address" : "192.168.1.100",
      "port": 80,
       "health_check_endpoint": "/healthcheck"
    },
    {
      "address" : "192.168.1.1001",
      "port": 80,
       "health_check_endpoint": "/healthcheck"
    }
  ]
}
```

and launch the load balancer with ``` zapLB ```

### How it works
---
The project is designed solely for learning Rust, so it's a simple load balancer. <br>
It operates using the round-robin technique to distribute requests to the backends. <br>
Upon launch, the load balancer performs a basic health check by making a call to an endpoint for each target defined in the configuration.

### Possible future implementations
---
- [x] pass the configuration file from CLI parameter
- [ ] HTTPS
- [ ] more flexible configuration
- [x] backend health check: basic health check at launch
- [ ] other balancing methods:
    - [ ] weighted least connection
    - [ ] adaptive load balancing
- [ ] Test code
