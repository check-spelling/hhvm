<?hh // partial

/**
 * Represents a connection to a set of memcached servers.
 */
<<__NativeData("MemcachedData")>>
class Memcached {
  // Signifies we have provide a session handler
  const HAVE_SESSION = false;
  /**
   * Create a Memcached instance
   *
   * @param string $persistent_id - By default the Memcached instances are
   * destroyed at the end of the request. To create an instance that persists
   * between requests, use persistent_id to specify a unique ID for the
   * instance. All instances created with the same persistent_id will share the
   * same connection.
   */
  <<__Native>>
  public function __construct(?string $persistent_id = null): void;

  /**
   * Add an item under a new key
   *
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key already exists.
   */
  public function add(mixed $key,
                      mixed $value,
                      int $expiration = 0): bool {
    return $this->addByKey('', $key, $value, $expiration);
  }

  /**
   * Add an item under a new key on a specific server
   *
   * @param string $server_key -
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key already exists.
   */
  <<__Native>>
  public function addByKey(string $server_key,
                           string $key,
                           mixed $value,
                           int $expiration = 0): bool;

  /**
   * Add a server to the server pool
   *
   * @param string $host - The hostname of the memcache server. If the
   *   hostname is invalid, data-related operations will set
   *   Memcached::RES_HOST_LOOKUP_FAILURE result code.
   * @param int $port - The port on which memcache is running. Usually,
   *   this is 11211.
   * @param int $weight - The weight of the server relative to the total
   *   weight of all the servers in the pool. This controls the probability
   *   of the server being selected for operations. This is used only with
   *   consistent distribution option and usually corresponds to the amount
   *   of memory available to memcache on that server.
   *
   * @return bool -
   */
  <<__Native>>
  public function addServer(string $host,
                            int $port,
                            int $weight = 0): bool;

  /**
   * Add multiple servers to the server pool
   *
   * @param array $servers -
   *
   * @return bool -
   */
  public function addServers(
    varray_or_darray<varray_or_darray<mixed>> $servers,
  ): bool {
    $servers_vals = array_values($servers);
    foreach($servers_vals as $i => $server) {
      $server = array_values($server);
      if (!\HH\is_any_array($server)) {
        trigger_error(
          sprintf('Server list entry #%d is not an array', $i + 1),
          E_WARNING
        );
        continue;
      }
      if (count($server) < 1) {
        trigger_error(
          sprintf('Could not get server host for entry #%d', $i + 1),
          E_WARNING
        );
        continue;
      }
      if (count($server) < 2) {
        trigger_error(
          sprintf('Could not get server port for entry #%d', $i + 1),
          E_WARNING
        );
        continue;
      }

      $host = (string)$server[0];
      $port = (int)$server[1];
      if (count($server) < 3) {
        $weight = 0;
      } else {
        $weight = (int)$server[2];
      }

      if (!$this->addServer($host, $port, $weight)) {
        trigger_error(
          sprintf('Could not add entry #%d to the server list', $i + 1),
          E_WARNING
        );
      }
    }
    return true;
  }

  /**
   * Append data to an existing item
   *
   * @param string $key -
   * @param string $value - The string to append.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  public function append(mixed $key,
                         mixed $value): bool {
    return $this->appendByKey('', $key, $value);
  }

  /**
   * Append data to an existing item on a specific server
   *
   * @param string $server_key -
   * @param string $key -
   * @param string $value - The string to append.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  <<__Native>>
  public function appendByKey(string $server_key,
                              string $key,
                              string $value): bool;

  /**
   * Compare and swap an item
   *
   * @param float $cas_token - Unique value associated with the existing
   *   item. Generated by memcache.
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_DATA_EXISTS if the item you are trying to store has
   *   been modified since you last fetched it.
   */
  public function cas(float $cas_token,
                      string $key,
                      mixed $value,
                      int $expiration = 0): bool {
    return $this->casByKey($cas_token, '', $key, $value, $expiration);
  }

  /**
   * Compare and swap an item on a specific server
   *
   * @param float $cas_token - Unique value associated with the existing
   *   item. Generated by memcache.
   * @param string $server_key -
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_DATA_EXISTS if the item you are trying to store has
   *   been modified since you last fetched it.
   */
  <<__Native>>
  public function casByKey(float $cas_token,
                           string $server_key,
                           string $key,
                           mixed $value,
                           int $expiration = 0): bool;

  /**
   * Decrement numeric item's value
   *
   * @param string $key - The key of the item to decrement.
   * @param int $offset - The amount by which to decrement the item's
   *   value.
   * @param mixed $initial_value - The value to set the item to if it
   *   doesn't currently exist. False to fail if the key does not exist
   * @param int $expiry - The expiry time to set on the item.
   *
   * @return mixed - Returns item's new value on success. False if the key
   *   doesn't exist and no initial_value was provided.
   */
  <<__Native>>
  public function decrement(string $key,
                            int $offset = 1,
                            mixed $initial_value = false,
                            int $expiry = 0): mixed;

  /**
   * Decrement numeric item's value, stored on a specific server
   *
   * @param string $server_key -
   * @param string $key - The key of the item to decrement.
   * @param int $offset - The amount by which to decrement the item's
   *   value.
   * @param int $initial_value - The value to set the item to if it
   *   doesn't currently exist. False to fail if the key does not exist.
   * @param int $expiry - The expiry time to set on the item.
   *
   * @return int - Returns item's new value on success. False if the key
   *   doesn't exist and no initial_value was provided.
   */
  <<__Native>>
  public function decrementByKey(string $server_key,
                                 string $key,
                                 int $offset = 1,
                                 mixed $initial_value = false,
                                 int $expiry = 0): mixed;

  /**
   * Delete an item
   *
   * @param string $key - The key to be deleted.
   * @param int $time - The amount of time the server will wait to delete
   *   the item.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTFOUND if the key does not exist.
   */
  public function delete(mixed $key,
                         int $time = 0): bool {
    return $this->deleteByKey('', $key, $time);
  }

  /**
   * Add an item under a new key on a specific server
   *
   * @param string $server_key - The key identifying the server to store the value on
   * or retrieve it from. Instead of hashing on the actual key for the item, we
   * hash on the server key when deciding which memcached server to talk to.
   * This allows related items to be grouped together on a single server for
   * efficiency with multi operations..
   * @param array $keys - The keys to be deleted.
   * @param int $time - The amount of time the server will wait to delete
   *   the items.
   *
   * @return array
   */
  <<__Native>>
  public function deleteMultiByKey(string $server_key, varray $keys,
                                   int $time = 0): mixed;

  /**
   * Add an item under a new key on a specific server
   *
   * @param array $keys - The keys to be deleted.
   * @param int $time - The amount of time the server will wait to delete
   *   the items.
   *
   * @return array
   */
  public function deleteMulti(varray $keys, int $time = 0): mixed {
    return $this->deleteMultiByKey('', $keys, $time);
  }

  /**
   * Delete an item from a specific server
   *
   * @param string $server_key -
   * @param string $key - The key to be deleted.
   * @param int $time - The amount of time the server will wait to delete
   *   the item.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTFOUND if the key does not exist.
   */
  <<__Native>>
  public function deleteByKey(string $server_key,
                              string $key,
                              int $time = 0): bool;

  /**
   * Fetch the next result
   *
   * @return array - Returns the next result or FALSE otherwise. The
   *   Memcached::getResultCode will return Memcached::RES_END if result
   *   set is exhausted.
   */
  <<__Native>>
  public function fetch(): mixed;

  /**
   * Fetch all the remaining results
   *
   * @return array - Returns the results.
   */
  <<__Native>>
  public function fetchAll(): mixed;

  /**
   * Invalidate all items in the cache
   *
   * @param int $delay - Number of seconds to wait before invalidating the
   *   items.
   *
   * @return bool -
   */
  <<__Native>>
  public function flush(int $delay = 0): bool;

  /**
   * Retrieve an item
   *
   * @param string $key - The key of the item to retrieve.
   * @param callable $cache_cb - Read-through caching callback or NULL.
   * @param float $cas_token - The variable to store the CAS token in.
   *
   * @return mixed - Returns the value stored in the cache or FALSE
   *   otherwise. The Memcached::getResultCode will return
   *   Memcached::RES_NOTFOUND if the key does not exist.
   */
  public function get(mixed $key,
                      ?mixed $cache_cb = null): mixed {
    return $this->getByKey('', $key, $cache_cb);
  }

  public function getWithCasToken(mixed $key,
                                  ?mixed $cache_cb,
                                  inout mixed $cas_token): mixed {
    $result = $this->getByKeyWithCasToken(
      '',
      $key,
      $cache_cb,
      inout $cas_token,
    );
    return $result;
  }

  /* Memcached::getAllKeys() Gets the keys stored on all the servers
   * @return mixed - Returns the keys stored on all the servers on success or
   * FALSE on failure.
   */
  <<__Native>>
  public function getAllKeys(): mixed;

  /**
   * Retrieve an item from a specific server
   *
   * @param string $server_key -
   * @param string $key - The key of the item to fetch.
   * @param mixed $cache_cb - Read-through caching callback or NULL
   * @param float $cas_token - The variable to store the CAS token in.
   *
   * @return mixed - Returns the value stored in the cache or FALSE
   *   otherwise. The Memcached::getResultCode will return
   *   Memcached::RES_NOTFOUND if the key does not exist.
   */
  <<__Native>>
  public function getByKey(string $server_key,
                           string $key,
                           mixed $cache_cb = null): mixed;

  <<__Native>>
  public function getByKeyWithCasToken(string $server_key,
                                       string $key,
                                       mixed $cache_cb,
                                       <<__OutOnly>>
                                       inout mixed $cas_token): mixed;

  /**
   * Request multiple items
   *
   * @param array $keys - Array of keys to request.
   * @param bool $with_cas - Whether to request CAS token values also.
   * @param callable $value_cb - The result callback or NULL.
   *
   * @return bool -
   */
  public function getDelayed(mixed $keys,
                             mixed $with_cas = false,
                             mixed $value_cb = null): bool {
    return $this->getDelayedByKey('', $keys, $with_cas, $value_cb);
  }

  /**
   * Request multiple items from a specific server
   *
   * @param string $server_key -
   * @param array $keys - Array of keys to request.
   * @param bool $with_cas - Whether to request CAS token values also.
   * @param callable $value_cb - The result callback or NULL.
   *
   * @return bool -
   */
  <<__Native>>
  public function getDelayedByKey(string $server_key,
                                  varray $keys,
                                  bool $with_cas = false,
                                  ?callable $value_cb = null): bool;

  /**
   * Retrieve multiple items
   *
   * @param array $keys - Array of keys to retrieve.
   * @param array $cas_tokens - The variable to store the CAS tokens for
   *   the found items.
   * @param int $flags - The flags for the get operation.
   *
   * @return mixed - Returns the array of found items.
   */
  public function getMulti(mixed $keys,
                           int $flags = 0): mixed {
    return $this->getMultiByKey('', $keys, $flags);
  }

  public function getMultiWithCasTokens(
    mixed $keys,
    inout mixed $cas_tokens,
    int $flags = 0,
  ): mixed {
    $result = $this->getMultiByKeyWithCasTokens(
      '',
      $keys,
      inout $cas_tokens,
      $flags,
    );
    return $result;
  }

  /**
   * Retrieve multiple items from a specific server
   *
   * @param string $server_key -
   * @param array $keys - Array of keys to retrieve.
   * @param string $cas_tokens - The variable to store the CAS tokens for
   *   the found items.
   * @param int $flags - The flags for the get operation.
   *
   * @return array - Returns the array of found items.
   */
  <<__Native>>
  public function getMultiByKey(string $server_key,
                                varray $keys,
                                int $flags = 0): mixed;

  <<__Native>>
  public function getMultiByKeyWithCasTokens(string $server_key,
                                             varray $keys,
                                             <<__OutOnly>>
                                             inout mixed $cas_tokens,
                                             int $flags = 0): mixed;

  /**
   * Retrieve a Memcached option value
   *
   * @param int $option - One of the Memcached::OPT_* constants.
   *
   * @return mixed - Returns the value of the requested option, or FALSE
   *   on error.
   */
  <<__Native>>
  public function getOption(int $option): mixed;

  /**
   * Return the result code of the last operation
   *
   * @return int - Result code of the last Memcached operation.
   */
  <<__Native>>
  public function getResultCode(): int;

  /**
   * Return the message describing the result of the last operation
   *
   * @return string - Message describing the result of the last Memcached
   *   operation.
   */
  <<__Native>>
  public function getResultMessage(): string;

  /**
   * Map a key to a server
   *
   * @param string $server_key -
   *
   * @return array - Returns an array containing three keys of host,
   *   port, and weight on success or FALSE on failure.
   */
  <<__Native>>
  public function getServerByKey(string $server_key): mixed;

  /**
   * Get the list of the servers in the pool
   *
   * @return array - The list of all servers in the server pool.
   */
  <<__Native>>
  public function getServerList(): varray;

  /**
   * Clears all server from the list
   *
   * @return bool - Returns TRUE on success or FALSE on failure.
   */
  <<__Native>>
  public function resetServerList(): bool;

  /**
   * Get server pool statistics
   *
   * @return array - Array of server statistics, one entry per server.
   */
  <<__Native>>
  public function getStats(): mixed;

  /**
   * Get server pool version info
   *
   * @return array - Array of server versions, one entry per server.
   */
  <<__Native>>
  public function getVersion(): mixed;

  /**
   * Increment numeric item's value
   *
   * @param string $key - The key of the item to increment.
   * @param int $offset - The amount by which to increment the item's
   *   value.
   * @param mixed $initial_value - The value to set the item to if it
   *   doesn't currently exist. False to fail if the key does not exist.
   * @param int $expiry - The expiry time to set on the item.
   *
   * @return mixed - Returns new item's value on success. False if the key
   * doesn't exist.
   */
  <<__Native>>
  public function increment(string $key,
                            int $offset = 1,
                            mixed $initial_value = false,
                            int $expiry = 0): mixed;

  /**
   * Increment numeric item's value, stored on a specific server
   *
   * @param string $server_key -
   * @param string $key - The key of the item to increment.
   * @param int $offset - The amount by which to increment the item's
   *   value.
   * @param mixed $initial_value - The value to set the item to if it
   *   doesn't currently exist. False to fail if the key does not exist.
   * @param int $expiry - The expiry time to set on the item.
   *
   * @return mixed - Returns new item's value on success. False if the key
   *   doesn't exist and no initial_value was provided.
   */
  <<__Native>>
  public function incrementByKey(string $server_key,
                                 string $key,
                                 int $offset = 1,
                                 mixed $initial_value = false,
                                 int $expiry = 0): mixed;

  /**
   * Check if a persitent connection to memcache is being used.
   *
   * @return bool - Returns true if Memcache instance uses a persistent
   * connection, false otherwise.
   */
  <<__Native>>
  public function isPersistent(): bool;

  /**
   * Check if the instance was recently created
   *
   * @return bool - Returns the true if instance is recently created,
   * false otherwise.
   */
  <<__Native>>
  public function isPristine(): bool;

  /**
   * Prepend data to an existing item
   *
   * @param string $key - The key of the item to prepend the data to.
   * @param string $value - The string to prepend.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  public function prepend(mixed $key,
                          mixed $value): bool {
    return $this->prependByKey('', $key, $value);
  }

  /**
   * Prepend data to an existing item on a specific server
   *
   * @param string $server_key -
   * @param string $key - The key of the item to prepend the data to.
   * @param string $value - The string to prepend.
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  <<__Native>>
  public function prependByKey(string $server_key,
                               string $key,
                               string $value): bool;

  /**
   * Memcached::quit() closes any open connections to the memcache servers.
   * @return bool TRUE on success or FALSE on failure
  */
  <<__Native>>
  public function quit(): bool;

  /**
   * Replace the item under an existing key
   *
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  public function replace(mixed $key,
                          mixed $value,
                          int $expiration = 0): bool {
    return $this->replaceByKey('', $key, $value, $expiration);
  }

  /**
   * Replace the item under an existing key on a specific server
   *
   * @param string $server_key -
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool - The Memcached::getResultCode will return
   *   Memcached::RES_NOTSTORED if the key does not exist.
   */
  <<__Native>>
  public function replaceByKey(string $server_key,
                               string $key,
                               mixed $value,
                               int $expiration = 0): bool;

  /**
   * Store an item
   *
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool -
   */
  public function set(mixed $key,
                      mixed $value,
                      int $expiration = 0): bool {
    return $this->setByKey('', $key, $value, $expiration);
  }

  /**
   * Store an item on a specific server
   *
   * @param string $server_key -
   * @param string $key -
   * @param mixed $value -
   * @param int $expiration -
   *
   * @return bool -
   */
  <<__Native>>
  public function setByKey(string $server_key,
                           string $key,
                           mixed $value,
                           int $expiration = 0): bool;

  /**
   * Store multiple items
   *
   * @param array $items -
   * @param int $expiration -
   *
   * @return bool -
   */
  public function setMulti(darray<string, mixed> $items,
                           int $expiration = 0): bool {
    return $this->setMultiByKey('', $items, $expiration);
  }

  /**
   * Store multiple items on a specific server
   *
   * @param string $server_key -
   * @param array $items -
   * @param int $expiration -
   *
   * @return bool -
   */
  public function setMultiByKey(string $server_key,
                                darray<string, mixed> $items,
                                int $expiration = 0): bool {
    foreach($items as $key => $value) {
      if (is_int($key)) {
        // numeric strings (e.g. '5') become integers as array keys
        $key = (string)$key;
      } else if (!is_string($key)) {
        continue;
      }
      if (!$this->setByKey($server_key, $key, $value, $expiration)) {
        return false;
      }
    }
    return true;
  }

  /**
   * Set a Memcached option
   *
   * @param int $option -
   * @param mixed $value -
   *
   * @return bool -
   */
  <<__Native>>
  public function setOption(int $option,
                            mixed $value): bool;

  /**
   * Set Memcached options
   *
   * @param array $options -
   *
   * @return bool -
   */
  public function setOptions(darray<int, mixed> $options): bool {
    foreach($options as $option => $value) {
      if (!$this->setOption($option, $value)) {
        return false;
      }
    }
    return true;
  }

  /**
   * Set a new expiration on an item
   *
   * @param string $key - The key under which to store the value.
   * @param int $expiration - The expiration time, defaults to 0.
   *
   * @return bool - Returns TRUE on success or FALSE on failure.
   */
  public function touch(string $key,
                        int $expiration = 0): bool {
    return $this->touchByKey('', $key, $expiration);
  }

  /**
   * Set a new expiration on an item on a specific server
   *
   * @param string $server_key - The key identifying the server to store the
   *   value on or retrieve it from. Instead of hashing on the actual key for
   *   the item, we hash on the server key when deciding which memcached server
   *   to talk to. This allows related items to be grouped together on a single
   *   server for efficiency with multi operations.
   * @param string $key - The key under which to store the value.
   * @param int $expiration - The expiration time, defaults to 0.
   *
   * @return bool - Returns TRUE on success or FALSE on failure.
   */
  <<__Native>>
  public function touchByKey(string $server_key,
                             string $key,
                             int $expiration = 0): bool;

}

class MemcachedException {
}
