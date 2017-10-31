## itu

if rust and moonscript copulated and had a super cool interpreted baby

### example

untested projection thing

```rust
projection := @{
  scale: f64
  point: [f64]
}

projection:
  new := (scale, point): projection ->
    projection {
      scale, point
    }

project := (fov: f64, point: [f64]): projection ->
  scale: f64        = fov / (fov + point[#point])
  point2: mut [f64] = {}
  
  for value in point
    array.push point, c * scale

    if #point2 - #point - 1 == 0 then break

  projection.new scale, point2

project_to := (dimension: i32, fov: f64, point: [f64]): projection ->
  projected := project fov, point
  
  if dimension - #point2 == 0
    projected
  else
    project_to dimension, fov, projected.point
```
