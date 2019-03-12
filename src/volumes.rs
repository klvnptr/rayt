use data::vector::Vector;
use view::Ray;

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64>;

    fn surface_normal(&self, ray: &Ray, hit_t: f64) -> Vector;
}

pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
}

impl Hitable for Sphere {

    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        // p(t) = ray
        // c = sphere_centre
        // R = sphere_radius
        // dot((p(t) - c), (p(t) - c)) = R^2

        let oc = ray.origin() - &self.centre;

        let a = Vector::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vector::dot(&oc, &ray.direction());
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Option::None
        }

        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < tmin || t > tmax {
            return Option::None
        }

        Option::Some(t)
    }

    fn surface_normal(&self, ray: &Ray, hit_t: f64) -> Vector {
        (ray.point(hit_t) - &self.centre).unit_vector()
    }
}
