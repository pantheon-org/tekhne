# LogQL Function and Parser Reference

## Quick Function Reference

### Log Range Aggregations (most common)

| Function | Description |
|----------|-------------|
| `rate(log-range)` | Entries per second |
| `count_over_time(log-range)` | Count entries |
| `bytes_rate(log-range)` | Bytes per second |
| `absent_over_time(log-range)` | Returns 1 if no logs |

### Unwrapped Range Aggregations (most common)

| Function | Description |
|----------|-------------|
| `sum_over_time`, `avg_over_time`, `max_over_time`, `min_over_time` | Aggregate numeric values |
| `quantile_over_time(φ, range)` | φ-quantile (0 ≤ φ ≤ 1) |
| `first_over_time`, `last_over_time` | First/last value |

### Aggregation Operators

`sum`, `avg`, `min`, `max`, `count`, `stddev`, `topk`, `bottomk`, `approx_topk`, `sort`, `sort_desc`

With grouping: `sum by (label1, label2)` or `sum without (label1)`

### Conversion Functions

| Function | Description |
|----------|-------------|
| `duration_seconds(label)` | Convert duration string |
| `bytes(label)` | Convert byte string (KB, MB) |

### label_replace()

```logql
label_replace(rate({job="api"} |= "err" [1m]), "foo", "$1", "service", "(.*):.*")
```

## Parser Reference

### logfmt

```logql
| logfmt [--strict] [--keep-empty]
```

- `--strict`: Error on malformed entries
- `--keep-empty`: Keep standalone keys

### JSON

```logql
| json                                           # All fields
| json method="request.method", status="response.status"  # Specific fields
| json servers[0], headers="request.headers[\"User-Agent\"]"  # Nested/array
```

## Template Functions

Common functions for `line_format` and `label_format`:

**String**: `trim`, `upper`, `lower`, `replace`, `trunc`, `substr`, `printf`, `contains`, `hasPrefix`
**Math**: `add`, `sub`, `mul`, `div`, `addf`, `subf`, `floor`, `ceil`, `round`
**Date**: `date`, `now`, `unixEpoch`, `toDate`, `duration_seconds`
**Regex**: `regexReplaceAll`, `count`
**Other**: `fromJson`, `default`, `int`, `float64`, `__line__`, `__timestamp__`

See `assets/common_queries.logql` for detailed usage examples.
