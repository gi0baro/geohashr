# Geohashr

*Just another Python geohashing library.*

## In a nutshell

```python
import geohashr

geohashr.encode(45.464664, 9.188540)
# 'u0nd9hdfue8h'

geohashr.decode('u0nd9hdfue8h')
# (45.46466396190226, 9.188540149480104)

geohashr.decode_exact('u0nd9hd')
# (45.46485900878906, 9.188003540039062, 0.0006866455078125, 0.0006866455078125)

geohashr.bbox('u0nd9hdfue8h')
# {
#     'e': 9.188540317118168,
#     's': 45.46466387808323,
#     'w': 9.188539981842041,
#     'n': 45.46466404572129
# }

geohashr.neighbors('u0nd9hdfue8h')
# {
#     'e': 'u0nd9hdfue8k',
#     'n': 'u0nd9hdfue8j',
#     'ne': 'u0nd9hdfue8m',
#     'nw': 'u0nd9hdfu7xv',
#     's': 'u0nd9hdfue85',
#     'se': 'u0nd9hdfue87',
#     'sw': 'u0nd9hdfu7xg',
#     'w': 'u0nd9hdfu7xu'
# }

geohashr.neighbor('u0nd9hdfue8h', 'e')
# 'u0nd9hdfue8k'
```

## License

Geohashr is released under the BSD License.
