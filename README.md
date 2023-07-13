# Geohashr

*Just another Python geohashing library.*

## In a nutshell

```python
import geohashr

geohashr.encode(45.464664, 9.188540)
# 't18k68dr5kn8'

geohashr.decode('t18k68dr5kn8')
# (45.46466404572129, 9.188540065661073)

geohashr.bbox('t18k68dr5kn8')
# {
#     'e': 45.464664213359356, 
#     's': 9.188539981842041, 
#     'w': 45.46466387808323, 
#     'n': 9.188540149480104
# }

geohashr.neighbors('t18k68dr5kn8')
# {
#     'e': 't18k68dr5knb', 
#     'n': 't18k68dr5kn9', 
#     'ne': 't18k68dr5knc', 
#     'nw': 't18k68dr5kn3', 
#     's': 't18k68dr57yx', 
#     'se': 't18k68dr57yz', 
#     'sw': 't18k68dr57yr', 
#     'w': 't18k68dr5kn2'
# }

geohashr.neighbor('t18k68dr5kn8', 'e')
# 't18k68dr5knb'
```

## License

Geohashr is released under the BSD License.
