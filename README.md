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
      "port": 80  
    },  
    {  
      "address" : "192.168.1.101",  
      "port": 80  
    }  
  ]  
}
```

and launch the load balancer with ``` zapLB ```

### How it works
---
The project is made with the unique purpose to learn Rust so it's a pretty basic load balancer,
it works using a round-robin technique to deliver the request to the backends.

### Possible future implementations
---
- [x] pass the configuration file from CLI parameter
- [ ] HTTPS
- [ ] more flexible configuration
- [ ] backend health check
- [ ] other balancing methods:
    - [ ] weighted least connection
    - [ ] adaptive load balancing
