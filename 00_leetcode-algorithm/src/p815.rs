use std::collections::{HashSet, LinkedList};

/// runs in O(mn + n^2)/O(n^2 + m)
pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }

    let n = routes.len();
    let (g, sources, targets) = {
        let routes = routes
            .into_iter()
            .map(|v| v.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let mut sources = vec![];
        let mut targets = HashSet::new();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            let ht1 = &routes[i];
            for j in (i + 1)..n {
                let ht2 = &routes[j];
                if !ht1.is_disjoint(ht2) {
                    g[i].push(j);
                    g[j].push(i);
                }
            }
            if ht1.contains(&source) {
                sources.push(i);
            }
            if ht1.contains(&target) {
                targets.insert(i);
            }
        }
        (g, sources, targets)
    };

    let mut visited = vec![false; n];
    let mut q = LinkedList::new();
    for x in sources.iter() {
        visited[*x] = true;
        q.push_back((*x, 1));
    }
    while !q.is_empty() {
        let mut nq = LinkedList::new();
        while let Some((nxt, d)) = q.pop_front() {
            if targets.contains(&nxt) {
                return d;
            } else {
                for &nxt in g[nxt].iter() {
                    if !visited[nxt] {
                        visited[nxt] = true;
                        nq.push_back((nxt, d + 1));
                    }
                }
            }
        }
        q = nq;
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_num_buses_to_destination() {
        let ret = num_buses_to_destination(
            [
                [24].into(),
                [3, 6, 11, 14, 22].into(),
                [1, 23, 24].into(),
                [0, 6, 14].into(),
                [1, 3, 8, 11, 20].into(),
            ]
            .into(),
            20,
            8,
        );
        assert_eq!(ret, 1);
    }
}

mod tle {
    use std::collections::{HashMap, HashSet};

    #[derive(Clone)]
    struct GraphNode {
        next: Vec<(usize, usize)>,
    }

    const N: usize = 1e6 as usize;

    /// runs in O(?)/O(?)
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        // 1. construct graph by adjacent list
        let mut g: Vec<GraphNode> = vec![GraphNode { next: vec![] }; N];
        for (id, r) in routes.iter().enumerate() {
            let mut i1 = r.iter();
            let mut i2 = r.iter().skip(1);
            while let Some((t1, t2)) = i1.next().zip(i2.next()) {
                g[(*t1 as usize)].next.push((*t2 as usize, id));
                g[(*t2 as usize)].next.push((*t1 as usize, id));
            }
        }

        // 2. dfs
        let mut vis_stops = HashSet::new();
        let mut vis_buses = HashMap::new();
        let ans = dfs(
            source as usize,
            target as usize,
            &g,
            &mut vis_stops,
            &mut vis_buses,
        );

        ans as i32
    }

    fn dfs(
        source: usize,
        target: usize,
        g: &[GraphNode],
        vis_stops: &mut HashSet<usize>,
        vis_buses: &mut HashMap<usize, usize>,
    ) -> usize {
        if source == target {
            return vis_buses.len();
        }

        vis_stops.insert(source);

        let mut min = usize::MAX;
        for &(nxt, bus) in &g[source].next {
            if !vis_stops.contains(&nxt) {
                vis_buses
                    .entry(bus)
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
                min = min.min(dfs(nxt, target, g, vis_stops, vis_buses));
                if let Some(t) = vis_buses.get_mut(&bus) {
                    if *t > 1 {
                        *t -= 1;
                    } else {
                        vis_buses.remove(&bus);
                    }
                }
            }
        }

        vis_stops.remove(&source);
        min
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn it_num_buses_to_destination() {
            let ret = num_buses_to_destination(
                [
                    [24].into(),
                    [3, 6, 11, 14, 22].into(),
                    [1, 23, 24].into(),
                    [0, 6, 14].into(),
                    [1, 3, 8, 11, 20].into(),
                ]
                .into(),
                20,
                8,
            );
            assert_eq!(ret, 1);
        }
    }
}
