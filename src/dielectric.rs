use crate::{
    color::Color, hittable::HitRecord, material::Material, material::ScatterResult, ray::Ray,
    utility, vector3::Vector3,
};

pub struct Dielectric {
    ir: f64, // Index of Refraction (for air is 1.0, glass somewhere between 1.3-1.7, diamond 2.4, etc)
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // We use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vector3::unit_vector(r_in.direction());
        let cos_theta = Vector3::dot(&(-unit_direction.clone()), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > utility::rand()
        {
            Vector3::reflect(&unit_direction, &rec.normal)
            // Vector3::new(0., 0. , 0.0)
        } else {
            Vector3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        // let direction = Vector3::reflect(&unit_direction, &rec.normal);
        let scattered = Ray::new(rec.point.clone(), direction);

        ScatterResult {
            success: true,
            attenuation,
            scattered,
        }
    }
}
