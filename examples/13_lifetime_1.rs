use std::collections::HashMap;

/// A config parser that stores references to key-value pairs
/// from an underlying config string (zero-copy design)
#[derive(Debug)]
struct ConfigCache<'a> {
    source: &'a str,
    cache: HashMap<&'a str, &'a str>,
}

impl<'a> ConfigCache<'a> {
    fn new(source: &'a str) -> Self {
        let mut cache = HashMap::new();

        // Parse lines like "key=value" and store references
        for line in source.lines() {
            if let Some((key, value)) = line.split_once('=') {
                cache.insert(key.trim(), value.trim());
            }
        }

        ConfigCache { source, cache }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.cache.get(key).copied()
    }

    /// Returns all keys that start with the given prefix
    fn keys_with_prefix(&self, prefix: &str) -> Vec<&str> {
        self.cache
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .map(|(&k, _)| k)
            .collect()
    }

    /// Returns a new ConfigCache containing only entries matching a predicate
    fn filter<F>(&self, predicate: F) -> ConfigCache<'a>
    where
        F: Fn(&str, &str) -> bool,
    {
        let filtered_cache = self
            .cache
            .iter()
            .filter(|(a, b)| predicate(a, b))
            .map(|(&k, &v)| (k, v))
            .collect();

        ConfigCache {
            source: self.source,
            cache: filtered_cache,
        }
    }
}

fn main() {
    let config = ConfigCache::new("timeout=11\ndebug_level=2\ndebug_size=300");
    println!("ConfigCache size: {}", config.cache.len());
    let debug_config: Vec<&str> = config.keys_with_prefix("debug");
    println!("Keys: {:?}", debug_config);
    let filtered = config.filter(|key, val| key == "timeout" || val == "300");
    println!("Filtered: {:?}", filtered);
    let got = config.get("timeout");
    println!("Got timeout {:?}", got);

    // demonstrate scope
    let conf;
    {
        let cfg = String::from("timeout=001");
        conf = ConfigCache::new(&cfg); //  `cfg` does not live long enough borrowed value does not live long enough
        println!("Config: {conf:?}");
    }
    // println!("Config: {conf:?}"); // this one produces compiler error from above: cfg does not live
    // long enough
}
