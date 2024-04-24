use cluster::Cluster;
use point::Point;

mod cluster;
mod point;

fn main() {
    let clusters = vec![
        Cluster::new((5, -3, 7)),
        Cluster::new((4, -8, 8)),
        Cluster::new((0, -6, 4)),
    ];

    let points: Vec<Point> = vec![
        (5, -3, 7).into(),
        (-7, 9, -5).into(),
        (-6, 9, -6).into(),
        (-6, 9, 8).into(),
        (6, -6, -6).into(),
        (-4, 0, 7).into(),
        (0, 5, 3).into(),
        (4, -8, 8).into(),
        (9, 1, 8).into(),
        (0, -6, 4).into(),
    ];

    let clusters = k_means(&points, clusters);
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}:", i + 1);
        println!("{cluster}\n");
    }
}

fn k_means<'a>(
    points: &'a Vec<Point>,
    mut clusters: Vec<Cluster<'a>>,
) -> Vec<Cluster<'a>> {
    let mut con = true;
    while con {
        con = false;
        for c in &mut clusters {
            c.clear();
        }

        for point in points {
            let cluster = closest_cluster(&clusters, point);
            clusters[cluster].add(point);
        }

        for cluster in &mut clusters {
            let dif = cluster.calc_center();
            if ((dif * 100.0).round() / 100.0) > 0.0 {
                con = true;
            }
        }
    }
    clusters
}

fn closest_cluster(clusters: &[Cluster], point: &Point) -> usize {
    let mut min = f64::INFINITY;
    let mut closest = 0;

    for (i, cluster) in clusters.iter().enumerate() {
        let dist = point.dist(&cluster.centroid);
        if dist < min {
            min = dist;
            closest = i;
        }
    }

    closest
}
