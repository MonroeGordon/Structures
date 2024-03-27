extern crate alloc;
extern crate core;

pub mod collection;
pub mod array;
pub mod queue;
pub mod stack;
pub mod set;
pub mod map;
pub mod grid;
pub mod supers;

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use crate::collection::*;
    use crate::map::*;
    use len_trait::*;
    use rand::Rng;
    use str_macro::*;
    use crate::queue::deque::*;
    use crate::grid::*;
    use crate::{dkv, kv};
    use crate::array::*;
    use crate::map::traversable::graph::*;
    use crate::map::traversable::linked::*;
    use crate::array::list::*;
    use crate::queue::*;
    use crate::set::*;
    use crate::stack::*;
    use crate::supers::*;
    use crate::map::traversable::*;
    use crate::map::traversable::tree::*;
    use crate::array::list::vector::*;

    #[test]
    fn adjacencylist_test() {
        let mut alist1: AdjacencyList<i8> = AdjacencyList::new();
        alist1.prepend(&LinkedList::from_vec(&vec![1, 2, 3]));
        alist1.insert(1, &LinkedList::from_vec(&vec![4, 5, 6]));
        alist1.append(&LinkedList::from_vec(&vec![7, 8, 9]));
        let mut alist2: AdjacencyList<i8> = alist1.clone();
        assert_eq!(alist1, alist2);
        assert!(alist1 == alist2);
        assert_eq!(alist1.len(), 3);
        assert!(alist1.capacity() >= 3);
        alist2.clear();
        assert!(alist2.is_empty());
        println!("{:?}", alist1);
        println!("AdjacencyList List 2: {:?}", alist1[1]);
        alist1[1].append(7);
        println!("AdjacencyList List 2: {:?}", alist1[1]);
        for i in alist1.clone().into_iter() {
            println!("{:?}", i);
        }
        for i in alist1.clone().to_vec() {
            print!("{:?} ", i);
        }
        println!();
        assert_eq!(alist1.get(1), Some(&alist1[1].clone()));
        assert_eq!(alist1.index_of(&alist1[1].clone()), Some(1));
        alist1.remove(1);
        assert!(!alist1.contains_all(&vec![kv!(0, 4), kv!(1, 5), kv!(2, 6)]));
        alist1.set(1, &LinkedList::from_vec(&vec![4, 5, 6]));
        assert!(alist1.contains_all(&vec![kv!(0, 4), kv!(1, 5), kv!(2, 6)]));
    }

    #[test]
    fn array_test() {
        let mut rng = rand::thread_rng();

        let mut arr1: Array<i8, 10> = Array::new();
        assert_eq!(arr1.capacity(), 10);
        assert!(arr1.contains(&0i8));
        assert!(arr1.contains_all(&arr1.to_vec()));
        assert_eq!(arr1.get(3), Some(&0));
        assert_eq!(arr1.index_list(&0), Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(arr1.index_of(&0), Some(0));
        assert_eq!(arr1.last_index_of(&0), Some(9));
        arr1.set(3, &2);
        assert_eq!(arr1.get(3), Some(&2));
        let slice1 = arr1.slice(3..10);
        assert_eq!(*slice1, [2i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8]);
        arr1.clear();
        assert_eq!(arr1.to_vec(), vec![0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8]);
        assert_eq!(arr1, arr1.clone());
        println!("{:?}", arr1);
        assert!(!arr1.is_empty());
        arr1[2] = 2i8;
        assert_eq!(arr1[2], 2i8);
        arr1[2] = 0i8;
        assert_eq!(arr1[2], 0i8);
        let mut len: usize = 0;
        for i in arr1.clone().into_iter() {
            assert_eq!(i, 0i8);
            len += 1;
        }
        assert_eq!(len, arr1.len());
        assert_eq!(arr1.len(), 10);
        assert!(arr1 == arr1);
        for i in 0..arr1.len() {
            arr1.set(i, &rng.gen::<i8>());
        }
        println!("{:?}", arr1);
        arr1.sort();
        assert!(arr1.is_sorted());
        for i in 0..arr1.len() {
            arr1.set(i, &rng.gen::<i8>());
        }
        println!("{:?}", arr1);
        arr1.sort_rev();
        assert!(arr1.is_sorted_rev());
        let mut arr2: Array<i8, 10> = Array::from_vec(&arr1.to_vec());
        assert_eq!(arr1, arr2);
        println!("Reversed: {:?}", arr2.reverse());
    }

    #[test]
    fn binarytree_test() {
        let mut tree1: BinaryTree<i32, i8, true> = BinaryTree::new();
        tree1.insert(KeyValue { key: 400, value: 1 });
        tree1.insert(KeyValue { key: 100, value: 2 });
        tree1.insert(KeyValue { key: 200, value: 3 });
        tree1.insert(KeyValue { key: 300, value: 4 });
        tree1.insert(KeyValue { key: 500, value: 5 });
        tree1.insert(KeyValue { key: 600, value: 6 });
        tree1.insert(KeyValue { key: 10, value: 7 });
        tree1.insert(KeyValue { key: 20, value: 8 });
        tree1.insert(KeyValue { key: 110, value: 9 });
        tree1.insert(KeyValue { key: 510, value: 10 });
        println!("{:?}", tree1);
        assert_eq!(tree1.len(), 10);
        let mut tree2: BinaryTree<i32, i8, true> = tree1.clone();
        tree2.clear();
        assert!(tree2.is_empty());
        assert_eq!(tree1[200], 3);
        tree1[300] = 14;
        assert_eq!(tree1[300], 14);
        tree1[300] = 4;
        for i in tree1.clone().into_iter() {
            print!("{}: {}, ", i.0, i.1);
        }
        println!();
        let mut trav = tree1.clone().into_trav();
        println!("Default Traversal (Inorder)");
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Level Order Traversal");
        trav.level_order();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Postorder Traversal");
        trav.postorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Preorder Traversal");
        trav.preorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Inorder Traversal");
        trav.inorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Boundary Traversal");
        trav.boundary();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Diagonal Traversal");
        trav.diagonal();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        assert!(tree1 == tree1);
        assert!(tree1.capacity() >= 10);
        assert!(tree1.contains(&KeyValue { key: 500, value: 5 }));
        assert!(tree1.contains_all(&tree1.clone().to_vec()));
        assert!(tree1.exists(300));
        assert!(!tree1.exists(1000));
        assert_eq!(tree1.get(500), Some(&5));
        assert_eq!(tree1.get(1000), None);
        assert!(tree1.remove(200));
        for i in tree1.clone().into_iter() {
            print!("{}: {}, ", i.0, i.1);
        }
        println!();
        tree1.replace(KeyValue { key: 510, value: 9 });
        assert_eq!(tree1[510], 9);
        println!("Degree of 400: {}", tree1.degree_of(400));
        println!("Edges: {}", tree1.edges());
        assert!(tree1.is_neighbor(400, 100));
        println!("Breadth: {}", tree1.breadth());
        println!("Child Nodes of 400: {:?}", tree1.child_nodes(&400));
        println!("Depth of 500: {}", tree1.depth_of(&500));
        println!("Level of 500: {}", tree1.level_of(&500));
        println!("Diameter: {}", tree1.diameter());
        println!("Height: {}", tree1.height());
        println!("Height from 100: {}", tree1.height_from(&100));
        assert!(tree1.is_ancestor(&100, &400));
        assert!(tree1.is_descendant(&400, &100));
        assert!(tree1.is_leaf(&510));
        assert!(tree1.is_sibling(&100, &500));
        assert_eq!(tree1.parent_node(&500), Some(&1));
        assert_eq!(tree1.root_node(), Some(&1));
        tree1.set_node(KeyValue { key: 510, value: 10 });
        assert_eq!(tree1[510], 10);
        let sub: BinaryTree<i32, i8, true> = tree1.subtree(100);
        println!("{:?}", sub);
        println!("Width of Level 1: {}", tree1.width(1));
        let tree3: BinaryTree<i32, i8, true> = BinaryTree::from_vec(&tree1.clone().to_vec());
        assert!(tree1.contains_all(&tree3.to_vec()));
        println!("Path: {:?}", tree1.path_of(400, 10));
    }

    #[test]
    fn deque_test() {
        let mut deq1: Deque<i8> = Deque::new();
        deq1.push(4);
        deq1.push(3);
        deq1.push(2);
        deq1.push(1);
        deq1.push(0);

        assert!(deq1.capacity() != 0);
        assert!(deq1.contains(&2));
        assert!(deq1.contains_all(&vec![0, 1, 2]));
        assert_eq!(deq1.to_vec(), vec![0, 1, 2, 3, 4]);
        assert_eq!(deq1.dequeue().expect("Deque dequeue failed."), 0);
        assert!(deq1.enqueue(0));
        assert_eq!(*deq1.peek().expect("Deque peek failed."), 1);
        assert_eq!(deq1.pop().expect("Deque pop failed."), 1);
        assert!(deq1.push(1));
        assert_eq!(*deq1.peek_top().expect("Deque peek top failed."), 1);
        assert_eq!(deq1.pop_last().expect("Deque pop last failed."), 0);
        assert_eq!(*deq1.peek_last().expect("Deque peek top failed."), 4);
        deq1.clear();
        assert!(deq1.is_empty());
        assert_eq!(deq1.len(), 0);
        assert!(!deq1.is_full());
        deq1.push(4);
        deq1.push(3);
        deq1.push(2);
        deq1.push(1);
        deq1.push(0);
        let deq2: Deque<i8> = deq1.clone();
        assert!(deq1 == deq2);
        for i in deq1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        println!("{:?}", deq1);
        let mut deq3: Deque<i8> = Deque::from_vec(&deq1.to_vec());
        assert_eq!(deq1, deq3);
        let deq5: Deque<i8> = Deque::with_capacity(20);
        assert_eq!(deq5.capacity(), 20);
        println!("Reversed: {:?}", deq3.reverse());
    }

    #[test]
    fn dictionary_test() {
        let mut dict1: Dictionary<i8> = Dictionary::new();
        dict1.insert(dkv!("One", 1));
        dict1.insert(dkv!("Two", 2));
        dict1.insert(dkv!("Three", 3));
        dict1.insert(dkv!("Four", 4));
        dict1.insert(dkv!("Five", 5));
        dict1.insert(dkv!("Six", 6));
        dict1.insert(dkv!("Seven", 7));
        dict1.insert(dkv!("Eight", 8));

        assert_eq!(dict1.capacity(), 8);
        assert!(dict1.contains(&KeyValue { key: str!("Three"), value: 3 }));
        assert!(dict1.contains_all(&dict1.to_vec()));
        assert!(dict1.exists(str!("Five")));
        assert_eq!(dict1.get(str!("Seven")), Some(&7));
        dict1.insert(KeyValue { key: str!("Nine"), value: 9 });
        assert_eq!(dict1.get(str!("Nine")), Some(&9));
        dict1.remove(str!("Six"));
        assert!(!dict1.exists(str!("Six")));
        dict1.replace(KeyValue { key: str!("Eight"), value: 10 });
        assert_eq!(dict1.get(str!("Eight")), Some(&10));
        let mut dict2: Dictionary<i8> = dict1.clone();
        assert_eq!(dict1, dict2);
        dict2.clear();
        assert_eq!(dict2.len(), 0);
        assert!(dict2.is_empty());
        println!("{:?}", dict1);
        for i in dict1.clone().into_iter() {
            assert_eq!(i.value, dict1[i.clone().key.to_string()]);
        }
        assert!(dict1 == dict1);
        dict1["Seven"] = 0;
        assert_eq!(dict1["Seven"], 0i8);
        dict1[str!("Five")] = 15;
        assert_eq!(dict1[str!("Five")], 15i8);
        dict2 = dict1.clone();
        assert_eq!(dict2, dict1);
        dict1.sort();
        assert!(dict1.is_sorted());
        dict1.sort_rev();
        assert!(dict1.is_sorted_rev());
        let dict3: Dictionary<i8> = Dictionary::from_vec(&dict1.to_vec());
        assert_eq!(dict1, dict3);
    }

    #[test]
    fn doublylinkedlist_test() {
        let mut dlist1: DoublyLinkedList<i8> = DoublyLinkedList::new();
        assert!(dlist1.insert(kv!(0, 1)));
        dlist1.append(2);
        dlist1.append(3);
        dlist1.append(4);
        dlist1.append(5);
        dlist1.prepend(0);
        let mut dlist2: DoublyLinkedList<i8> = dlist1.clone();
        assert_eq!(dlist1, dlist2);
        dlist2.clear();
        assert!(dlist2.is_empty());
        assert_eq!(dlist1.len(), 6);
        println!("{:?}", dlist1);
        for i in dlist1.clone().into_iter() {
            print!("{}: {}, ", i.key, i.value);
        }
        println!();
        let mut trav = dlist1.clone().into_trav();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from doubly linked list."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving previous node from doubly linked list."));
        }
        println!();
        assert!(dlist1 == dlist1);
        assert_eq!(dlist1.capacity(), dlist1.len());
        assert!(dlist1.contains(&KeyValue { key: 3, value: 3 }));
        assert!(dlist1.contains_all(&vec![kv!(0, 0), kv!(1, 1), kv!(2, 2)]));
        dlist1.circular(true);
        assert!(dlist1.is_circular());
        dlist1.remove(5);
        assert!(!dlist1.contains(&KeyValue { key: 5, value: 5 }));
        assert!(!dlist1.has_value(5));
        println!("{:?}", dlist1);
        let mut dlist3: DoublyLinkedList<i8> = DoublyLinkedList::new_circular();
        assert!(dlist3.is_circular());
        dlist3 = DoublyLinkedList::circular_from_vec(&vec![0, 1, 2, 3]);
        assert!(dlist3.is_circular());
        assert!(dlist3.contains_all(&vec![kv!(0, 0), kv!(1, 1), kv!(2, 2), kv!(3, 3)]));
        dlist3 = DoublyLinkedList::from_vec(&vec![0, 1, 2, 3, 4]);
        assert!(!dlist3.is_circular());
        assert!(dlist3.contains_all(&vec![kv!(0, 0), kv!(1, 1), kv!(2, 2), kv!(3, 3), kv!(4, 4)]));
        println!("Reversed: {:?}", dlist3.reverse());
        println!("Path: {:?}", dlist3.path_of(1, 3));
    }

    #[test]
    fn graph_test() {
        let mut g1: DWGraph<i8> = Graph::new();
        g1.insert(kv!(0, 5));
        g1.insert(kv!(1, 3));
        g1.insert(kv!(2, 7));
        g1.insert(kv!(3, 6));
        g1.connect(DWGraphEdge::new(0, 1, 0.5));
        g1.connect(DWGraphEdge::new(0, 2, 0.3));
        g1.connect(DWGraphEdge::new(2, 3, 1.0));
        g1.connect(DWGraphEdge::new(2, 1, -0.75));
        assert_eq!(g1, g1.clone());
        println!("{:?}", g1);
        assert!(!g1.is_empty());
        g1[1] = 4;
        assert_eq!(g1[1], 4);
        for i in g1.clone().into_iter() {
            print!("{:?} ", i);
        }
        println!();
        let mut g1t = g1.clone().into_trav();
        while g1t.has_next() {
            print!("{:?} ", g1t.next().unwrap());
        }
        println!();
        while g1t.has_prev() {
            print!("{:?} ", g1t.prev().unwrap());
        }
        println!();
        g1t.bfs_all();
        while g1t.has_next() {
            print!("{:?} ", g1t.next().unwrap());
        }
        println!();
        while g1t.has_prev() {
            print!("{:?} ", g1t.prev().unwrap());
        }
        println!();
        g1t.dfs();
        while g1t.has_next() {
            print!("{:?} ", g1t.next().unwrap());
        }
        println!();
        while g1t.has_prev() {
            print!("{:?} ", g1t.prev().unwrap());
        }
        println!();
        g1t.dfs_all();
        while g1t.has_next() {
            print!("{:?} ", g1t.next().unwrap());
        }
        println!();
        while g1t.has_prev() {
            print!("{:?} ", g1t.prev().unwrap());
        }
        println!();
        assert_eq!(g1.len(), 4);
        assert!(g1 == g1.clone());
        assert_eq!(g1.capacity(), 4);
        assert!(g1.contains(&g1.node(2).unwrap()));
        assert!(g1.contains_all(&g1.clone().to_vec()));
        assert!(g1.exists(3));
        assert_eq!(g1.get(2), Some(&7));
        g1.remove(3);
        assert!(!g1.exists(3));
        g1.replace(kv!(2, 8));
        assert_eq!(g1[2], 8);
        assert_eq!(g1.degree_of(0), 2);
        println!("Graph Diameter: {}", g1.diameter());
        println!("{:?}", g1.edge_list());
        assert_eq!(g1.edges(), 3);
        assert_eq!(g1.has_cycle(), false);
        assert_eq!(g1.is_bipartite(), false);
        assert_eq!(g1.is_connected(), false);
        assert!(g1.is_neighbor(0, 1));
        println!("{:?}", g1.path_of(0, 2));
        println!("{:?}", g1.center());
        println!("Graph Distance From 0 to 2: {}",
                 g1.distance(&g1.node(0).unwrap(), &g1.node(2).unwrap()).unwrap());
        println!("Eccentricity of Node 0: {}", g1.eccentricity(&g1.node(0).unwrap()).unwrap());
        println!("Edge from 0 to 2: {}", g1.edge(&g1.node(0).unwrap(), &g1.node(2).unwrap()));
        assert!(g1.has_neg_edges());
        println!("Graph Radius: {}", g1.radius());

        let mut g2: UUGraph<i8> = Graph::new();
        g2.insert(kv!(0, 5));
        g2.insert(kv!(1, 3));
        g2.insert(kv!(2, 7));
        g2.insert(kv!(3, 6));
        g2.connect(UUGraphEdge::new(0, 1));
        g2.connect(UUGraphEdge::new(0, 2));
        g2.connect(UUGraphEdge::new(2, 3));
        g2.connect(UUGraphEdge::new(2, 1));
        assert_eq!(g2, g2.clone());
        println!("{:?}", g2);
        assert!(!g2.is_empty());
        g2[1] = 4;
        assert_eq!(g2[1], 4);
        for i in g2.clone().into_iter() {
            println!("{:?}", i);
        }
        println!();
        let mut g2t = g2.clone().into_trav();
        while g2t.has_next() {
            print!("{:?} ", g2t.next().unwrap());
        }
        println!();
        while g2t.has_prev() {
            print!("{:?} ", g2t.prev().unwrap());
        }
        println!();
        g2t.bfs_all();
        while g2t.has_next() {
            print!("{:?} ", g2t.next().unwrap());
        }
        println!();
        while g2t.has_prev() {
            print!("{:?} ", g2t.prev().unwrap());
        }
        println!();
        g2t.dfs();
        while g2t.has_next() {
            print!("{:?} ", g2t.next().unwrap());
        }
        println!();
        while g2t.has_prev() {
            print!("{:?} ", g2t.prev().unwrap());
        }
        println!();
        g2t.dfs_all();
        while g2t.has_next() {
            print!("{:?} ", g2t.next().unwrap());
        }
        println!();
        while g2t.has_prev() {
            print!("{:?} ", g2t.prev().unwrap());
        }
        println!();
        assert_eq!(g2.len(), 4);
        assert!(g2 == g2.clone());
        assert_eq!(g2.capacity(), 4);
        assert!(g2.contains(&g2.node(2).unwrap()));
        assert!(g2.contains_all(&g2.clone().to_vec()));
        assert!(g2.exists(3));
        assert_eq!(g2.get(2), Some(&7));
        g2.remove(3);
        assert!(!g2.exists(3));
        g2.replace(kv!(2, 8));
        assert_eq!(g2[2], 8);
        assert_eq!(g2.degree_of(0), 2);
        println!("Graph Diameter: {}", g2.diameter());
        println!("{:?}", g2.edge_list());
        assert_eq!(g2.edges(), 3);
        assert_eq!(g2.has_cycle(), true);
        assert_eq!(g2.is_bipartite(), false);
        assert_eq!(g2.is_connected(), true);
        assert!(g2.is_neighbor(0, 1));
        println!("{:?}", g2.path_of(0, 2));
        println!("{:?}", g2.center());
        println!("Graph Distance From 0 to 2: {}",
                 g2.distance(&g2.node(0).unwrap(), &g2.node(2).unwrap()).unwrap());
        println!("Eccentricity of Node 0: {}", g2.eccentricity(&g2.node(0).unwrap()).unwrap());
        println!("Edge from 0 to 2: {}", g2.edge(&g2.node(0).unwrap(), &g2.node(2).unwrap()));
        assert!(!g2.has_neg_edges());
        println!("Graph Radius: {}", g2.radius());
    }

    #[test]
    fn grid_test() {
        let mut grid1: Grid<i8> = Grid::new();
        grid1.resize(5, 6);
        assert_eq!(grid1.rows(), 5);
        assert_eq!(grid1.col_size(), grid1.rows());
        assert_eq!(grid1.columns(), 6);
        assert_eq!(grid1.row_size(), grid1.columns());
        assert_eq!(grid1.len(), 30);
        assert_eq!(grid1.size(), 30);
        assert_eq!(grid1.capacity(), 30);
        grid1.insert_row(2);
        assert_eq!(grid1.rows(), 6);
        grid1.insert_row_val(4, &vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(*grid1.get(Pos::at(4, 2)).expect("Failed to get grid value"), 3);
        grid1[(4, 4)] = 8;
        assert_eq!(grid1[(4, 4)], 8);
        grid1.insert_col(3);
        assert_eq!(grid1.columns(), 7);
        grid1.insert_col_val(2, &vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(grid1[(4, 2)], 5);
        assert_eq!(grid1.pos_of(7).expect("Failed to get position of grid value"),
                   Pos::at(6, 2));
        assert_eq!(grid1.pos_list(4).expect("Failed to get position list of grid value"),
                   vec![Pos::at(3, 2), Pos::at(4, 5)]);
        assert!(grid1.contains(&8));
        assert!(grid1.contains_all(&vec![1, 2, 3, 4, 5, 6, 7, 8]));
        let mut grid2: Grid<i8> = grid1.clone();
        assert!(grid1 == grid2);
        grid2.clear();
        assert_eq!(grid2.size(), 0);
        assert!(grid2.is_empty());
        println!("{:?}", grid1);
        grid1.remove_col(3);
        println!("{:?}", grid1);
        grid1.remove_row(4);
        println!("{:?}", grid1);
        for i in grid1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        for i in grid1.to_vec() {
            print!("{} ", i);
        }
        println!();
        let vec1: Vec<i8> = grid1.get_col(2).expect("Failed to get column from grid.");
        for i in 0..vec1.len() {
            assert_eq!(vec1[i], grid1[(i, 2)]);
        }
        let vec2: Vec<i8> = grid1.get_row(4).expect("Failed to get row from grid.");
        for i in 0..vec1.len() {
            assert_eq!(vec2[i], grid1[(4, i)]);
        }
        let grid3: Grid<i8> = Grid::new_def(5, 5, 3);
        assert_eq!(grid3.size(), 25);
        assert!(grid3.contains(&3));
        let grid4: Grid<i8> = Grid::new_size(6, 6);
        assert_eq!(grid4.size(), 36);
        let grid5: Grid<i8> = Grid::from_vec(grid1.columns(), grid1.rows(), &grid1.to_vec());
        assert_eq!(grid5, grid1);
    }

    #[test]
    fn hashmap_test() {
        let mut hmap1: HashMap<i32, f32> = HashMap::new();
        assert!(hmap1.insert(kv!(0, 3.2)));
        assert!(hmap1.insert(kv!(1, 1.5)));
        assert!(hmap1.insert(kv!(2, 6.7)));
        assert!(hmap1.insert(kv!(3, 2.9)));
        assert!(hmap1.insert(kv!(4, 5.4)));
        assert!(hmap1.capacity() > 0);
        assert!(hmap1.contains(&kv!(1, 1.5)));
        assert!(hmap1.contains_all(&vec![kv!(0, 3.2), kv!(1, 1.5), kv!(2, 6.7)]));
        assert_eq!(hmap1, hmap1.clone());
        assert_eq!(hmap1.len(), 5);
        for i in hmap1.clone().into_iter() {
            assert_eq!(hmap1[i.key], i.value);
        }
        hmap1[3] = 4.6;
        assert_eq!(hmap1[3], 4.6);
        assert!(!hmap1.is_empty());
        println!("{:?}", hmap1);
        assert!(hmap1.exists(3));
        assert_eq!(*hmap1.get(3).expect("Failed to get hash map value"), 4.6);
        assert!(hmap1.remove(1));
        assert!(hmap1.replace(kv!(2, 3.8)));
        let hmap2: HashMap<i32, f32> = HashMap::from_vec(&hmap1.to_vec());
        assert_eq!(hmap1, hmap2);
    }

    #[test]
    fn hashset_test() {
        let mut hset1: HashSet<i8> = HashSet::new();
        hset1.add(0);
        hset1.add(1);
        hset1.add(2);
        assert!(hset1.capacity() > 0);
        assert!(hset1.contains(&1));
        assert!(hset1.contains_all(&vec![0, 1, 2]));
        let vec: Vec<i8> = hset1.to_vec();
        assert!(vec.contains(&0) && vec.contains(&1) && vec.contains(&2));
        let mut hset2: HashSet<i8> = HashSet::from_vec(&vec![3, 4, 5]);
        hset1.add_all(hset2.clone().to_vec());
        assert!(hset1.contains_all(&hset2.clone().to_vec()));
        hset1.remove(0);
        assert!(!hset1.contains(&0));
        hset1.remove_all(hset2.clone().to_vec());
        assert!(!hset1.contains_all(&hset2.clone().to_vec()));
        hset1.retain_all(hset1.clone().to_vec());
        assert!(hset1.contains_all(&hset1.clone().to_vec()));
        hset2.clear();
        assert!(hset2.is_empty());
        assert_eq!(hset1.len(), 2);
        println!("{:?}", hset1);
        for i in hset1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        assert!(hset1 == hset1);
    }

    #[test]
    fn linkedlist_test() {
        let mut llist1: LinkedList<i8> = LinkedList::new();
        assert!(llist1.insert(kv!(0, 1)));
        llist1.append(2);
        llist1.append(3);
        llist1.append(4);
        llist1.append(5);
        llist1.prepend(0);
        let mut llist2: LinkedList<i8> = llist1.clone();
        assert_eq!(llist1, llist2);
        llist2.clear();
        assert!(llist2.is_empty());
        assert_eq!(llist1.len(), 6);
        println!("{:?}", llist1);
        for i in llist1.clone().into_iter() {
            print!("{}: {}, ", i.0, i.1);
        }
        println!();
        let mut trav = llist1.clone().into_trav();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from linked list."));
        }
        println!();
        assert!(llist1 == llist1);
        assert_eq!(llist1.capacity(), llist1.len());
        assert!(llist1.contains(&kv!(3, 3)));
        assert!(llist1.contains_all(&llist1.clone().to_vec()));
        llist1.circular(true);
        assert!(llist1.is_circular());
        llist1.remove(5);
        assert!(!llist1.contains(&kv!(5, 5)));
        assert!(!llist1.has_value(5));
        println!("{:?}", llist1);
        let mut llist3: LinkedList<i8> = LinkedList::new_circular();
        assert!(llist3.is_circular());
        llist3 = LinkedList::circular_from_vec(&vec![0, 1, 2, 3]);
        assert!(llist3.is_circular());
        assert!(llist3.contains_all(&vec![kv!(0, 0), kv!(1, 1), kv!(2, 2), kv!(3, 3)]));
        llist3 = LinkedList::from_vec(&vec![0, 1, 2, 3, 4]);
        assert!(!llist3.is_circular());
        println!("Reversed: {:?}", llist3.reverse());
        println!("Path: {:?}", llist3.path_of(1, 3));
    }

    #[test]
    fn list_test() {
        let mut list1: List<i8> = List::new();
        list1.append(0);
        list1.append(1);
        list1.append(2);
        list1.append(3);
        list1.append(4);
        let mut list2: List<i8> = list1.clone();
        assert_eq!(list1, list2);
        list2.clear();
        assert!(list2.is_empty());
        assert_eq!(list1.len(), 5);
        println!("{:?}", list1);
        list1[3] = 5;
        assert_eq!(list1[3], 5);
        list1[3] = 3;
        for i in list1.clone().into_iter() {
            assert_eq!(list1[i as usize], i);
        }
        assert!(list1 == list1);
        let mut list2: List<i8> = List::new();
        list2.append(5);
        list2.append(6);
        list2.append(7);
        list2.append(8);
        list2.append(9);
        list2.sort();
        assert!(list2.is_sorted());
        list2.sort_rev();
        assert!(list2.is_sorted_rev());
        assert!(list1.capacity() > 0);
        assert!(list1.contains(&2));
        assert!(list1.contains_all(&vec![0, 1, 2]));
        assert_eq!(list1.get(2), Some(&2));
        assert_eq!(list1.index_list(&7), None);
        assert_eq!(list1.index_of(&4), Some(4));
        assert_eq!(list1.last_index_of(&3), Some(3));
        list1.set(0, &6);
        assert_eq!(list1[0], 6);
        let slice1 = list1.slice(2..5);
        assert_eq!(*slice1, [2i8, 3i8, 4i8]);
        list1.append_all(list2.clone().to_vec());
        for i in 5..list1.len() {
            assert_eq!(list1[i], list2[i - 5]);
        }
        list1.insert(5, 10);
        assert_eq!(list1[5], 10);
        list1.insert_all(2, list2.clone().to_vec());
        for i in 2..(list2.len() + 2) {
            assert_eq!(list1[i], list2[i - 2]);
        }
        list1.prepend(20);
        assert_eq!(list1[0], 20);
        list1.prepend_all(list1.clone().to_vec());
        for i in 0..(list1.len() / 2) {
            assert_eq!(list1[i], list1[i + (list1.len() / 2)]);
        }
        list1.remove(20);
        assert!(list1.contains(&20));
        list1.remove_last(20);
        assert!(!list1.contains(&20));
        list1.remove_all(list2.clone().to_vec());
        assert!(!list1.contains_all(&list2.clone().to_vec()));
        list1.remove_any(1);
        assert!(!list1.contains(&1));
        let mut list3: List<i8> = List::new();
        list3.append(2);
        list3.append(3);
        list3.append(4);
        list1.retain_all(list3.clone().to_vec());
        assert!(list1.contains_all(&list3.clone().to_vec()));
        let mut list4: List<i8> = List::from_vec(&list1.clone().to_vec());
        assert_eq!(list1, list4);
        println!("Reversed: {:?}", list4.reverse());
    }

    #[test]
    fn map_test() {
        let mut map1: Map<i32, f32> = Map::new();
        assert!(map1.insert(kv!(0, 3.2)));
        assert!(map1.insert(kv!(1, 1.5)));
        assert!(map1.insert(kv!(2, 6.7)));
        assert!(map1.insert(kv!(3, 2.9)));
        assert!(map1.insert(kv!(4, 5.4)));
        assert!(map1.capacity() > 0);
        assert!(map1.contains(&kv!(1, 1.5)));
        assert!(map1.contains_all(&vec![kv!(0, 3.2), kv!(1, 1.5), kv!(2, 6.7)]));
        assert_eq!(map1, map1.clone());
        assert_eq!(map1.len(), 5);
        for i in map1.clone().into_iter() {
            assert_eq!(map1[i.key], i.value);
        }
        map1[3] = 4.6;
        assert_eq!(map1[3], 4.6);
        assert!(!map1.is_empty());
        println!("{:?}", map1);
        assert!(map1.exists(3));
        assert_eq!(*map1.get(3).expect("Failed to get hash map value"), 4.6);
        assert!(map1.remove(1));
        assert!(map1.replace(kv!(2, 3.8)));
        map1.sort();
        assert!(map1.is_sorted());
        map1.sort_rev();
        assert!(map1.is_sorted_rev());
        let map2: Map<i32, f32> = Map::from_vec(&map1.to_vec());
        assert_eq!(map1, map2);
    }

    #[test]
    fn queue_test() {
        let mut q1: Queue<i8> = Queue::new();
        assert!(q1.enqueue(0));
        assert!(q1.enqueue(1));
        assert!(q1.enqueue(2));
        assert!(q1.enqueue(3));
        assert!(q1.enqueue(4));
        assert!(q1.capacity() > 0);
        assert!(q1.contains(&1));
        assert!(q1.contains_all(&vec![0, 1, 2]));
        assert_eq!(q1, q1.clone());
        assert_eq!(q1.len(), 5);
        for i in q1.clone().into_iter() {
            assert_eq!(q1.dequeue(), Some(i));
        }
        assert!(q1.is_empty());
        assert!(q1.enqueue(0));
        assert!(q1.enqueue(1));
        assert!(q1.enqueue(2));
        assert!(q1.enqueue(3));
        assert!(q1.enqueue(4));
        println!("{:?}", q1);
        assert_eq!(q1.peek(), Some(&0));
        let mut q2: Queue<i8> = Queue::from_vec(&q1.clone().to_vec());
        assert_eq!(q1, q2);
        let q3: Queue<i8> = Queue::with_capacity(10);
        assert_eq!(q3.capacity(), 10);
        println!("Reversed: {:?}", q2.reverse());
    }

    #[test]
    fn set_test() {
        let mut set1: Set<i8> = Set::new();
        set1.add(0);
        set1.add(1);
        set1.add(2);
        assert!(set1.capacity() > 0);
        assert!(set1.contains(&1));
        assert!(set1.contains_all(&vec![0, 1, 2]));
        let vec: Vec<i8> = set1.to_vec();
        assert!(vec.contains(&0) && vec.contains(&1) && vec.contains(&2));
        let mut set2: Set<i8> = Set::from_vec(&vec![3, 4, 5]);
        set1.add_all(set2.clone().to_vec());
        assert!(set1.contains_all(&set2.clone().to_vec()));
        set1.remove(0);
        assert!(!set1.contains(&0));
        set1.remove_all(set2.clone().to_vec());
        assert!(!set1.contains_all(&set2.clone().to_vec()));
        set1.retain_all(set1.clone().to_vec());
        assert!(set1.contains_all(&set1.clone().to_vec()));
        set2.clear();
        assert!(set2.is_empty());
        assert_eq!(set1.len(), 2);
        println!("{:?}", set1);
        for i in set1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        assert!(set1 == set1);
        let set3: Set<i8> = Set::from_vec(&set1.clone().to_vec());
        assert_eq!(set1, set3);
        let set4: Set<i8> = Set::not_from_vec(&set1.clone().to_vec());
        assert_ne!(set1, set4);
        let set5: Set<i8> = Set::with_capacity(10);
        assert_eq!(set5.capacity(), 10);
        let mut seta: Set<i8> = Set::new();
        seta.add(1);
        seta.add(2);
        seta.add(3);
        let mut setb: Set<i8> = Set::new_inf();
        let mut setc: Set<i8> = Set::intersection_of(&seta, &setb);
        assert_eq!(setc, seta);
        println!("{:?}", seta);
        println!("{:?}", setb);
        println!("{:?}", setc);
        setc = Set::union_of(&seta, &setb);
        assert_eq!(setc, setb);
        println!("{:?}", setc);
        setc = Set::difference_of(&setb, &seta);
        assert_eq!(setc, Set::complement_of(&seta));
        println!("{:?}", setc);
        assert!(setb.is_infinite());
        assert!(setc.is_complement());
        assert!(seta.is_finite());
        setb.complement();
        assert!(!setb.is_complement());
    }

    #[test]
    fn stack_test() {
        let mut rng = rand::thread_rng();

        let mut stack1: Stack<i8> = Stack::new();
        stack1.push(1);
        stack1.push(2);
        stack1.push(3);
        assert_eq!(stack1.len(), 3);
        let mut stack2: Stack<i8> = stack1.clone();
        assert_eq!(stack1, stack2);
        stack2.clear();
        assert!(stack2.is_empty());
        while !stack2.is_full() {
            stack2.push(rng.gen::<i8>());
        }
        assert_eq!(stack2.len(), stack2.capacity());
        println!("{:?}", stack2);
        for i in stack2.clone().into_iter() {
            print!("[ {} ]\n", i);
        }
        assert!(stack2 == stack2);
        assert!(stack1.contains(&2));
        assert!(stack2.contains_all(&stack2.clone().to_vec()));
        assert_eq!(stack1.pop(), Some(1));
        assert_eq!(stack1.peek_top(), Some(&2));
        let stack3: Stack<i8> = Stack::from_vec(&stack1.clone().to_vec());
        assert_eq!(stack3, stack1);
        let mut stack4: Stack<i8> = Stack::with_capacity(10);
        while !stack4.is_full() {
            stack4.push(rng.gen::<i8>());
        }
        println!("{:?}", stack4);
        assert_eq!(stack4.len(), stack4.capacity());
    }

    #[test]
    fn superlist_test() {
        let mut slist1: SuperList<i8> = SuperList::new();
        slist1.prepend(&List::from_vec(&vec![1, 2, 3]));
        slist1.insert(1, &List::from_vec(&vec![4, 5, 6]));
        slist1.append(&List::from_vec(&vec![7, 8, 9]));
        let mut slist2: SuperList<i8> = slist1.clone();
        assert_eq!(slist1, slist2);
        assert!(slist1 == slist2);
        assert_eq!(slist1.len(), 3);
        assert!(slist1.capacity() >= 3);
        slist2.clear();
        assert!(slist2.is_empty());
        println!("{:?}", slist1);
        println!("SuperList List 2: {:?}", slist1[1]);
        slist1[1].append(7);
        println!("SuperList List 2: {:?}", slist1[1]);
        for i in slist1.clone().into_iter() {
            println!("{:?}", i);
        }
        for i in slist1.clone().to_vec() {
            print!("{} ", i);
        }
        println!();
        assert_eq!(slist1.get(1), Some(&slist1[1].clone()));
        assert_eq!(slist1.index_of(&slist1[1].clone()), Some(1));
        slist1.remove(1);
        assert!(!slist1.contains_all(&vec![4, 5, 6]));
        slist1.set(1, &List::from_vec(&vec![4, 5, 6]));
        assert!(slist1.contains_all(&vec![4, 5, 6]));
    }

    #[test]
    fn table_test() {
        #[allow(unused_assignments)]
        let mut t1: Table = Table::new();
        t1 = Table::new_size(4, 5);
        println!("{}", t1);
        t1.set_col_headers(vec!(str!("Col 1"), str!("Col 2"), str!("Col 3"), str!("Col 4"), str!("Col 5")));
        println!("{}", t1);
        t1.set_row_headers(vec!(str!("Row 1"), str!("Row 2"), str!("Row 3"), str!("Row 4")));
        println!("{}", t1);
        t1.set_col_header(1, "Really Long Column Name");
        println!("{}", t1);
        t1.set_row_header(2, "Really Long Row Name");
        println!("{}", t1);
        t1.no_col_headers();
        println!("{}", t1);
        t1.no_row_headers();
        println!("{}", t1);
        t1 = Table::from_vec(3, 2,
                             &vec!(
                                 CellType::Integer(5),
                                 CellType::Float(3.2),
                                 CellType::String(str!("Hi")),
                                 CellType::Empty,
                                 CellType::LocalDateTime(DateTime::default()),
                                 CellType::UTCDateTime(DateTime::default())));
        println!("{}", t1);
        t1.set_col_headers(vec!(str!("Value 1"), str!("Value 2")));
        println!("{}", t1);
        t1.set_row_headers(vec!(str!("Int/Float"), str!("String/Empty"), str!("Local/UTC")));
        println!("{}", t1);
        t1.no_headers();
        println!("{}", t1);
        assert_eq!(t1.columns(), 2);
        assert_eq!(t1.rows(), 3);
        assert_eq!(t1.col_size(), t1.rows());
        assert_eq!(t1.row_size(), t1.columns());
        match t1.get(Pos::at(1, 0)) {
            Some(cell) => println!("Cell at ({}, {}): {}", 1, 0, cell),
            None => panic!("Unexpected error retrieving table cell."),
        }
        t1.set(Pos::at(1, 0), Cell::new_data(CellType::String(str!("Bye"))));
        println!("{}", t1);
        match t1.get_row(1) {
            Some(r) => println!("{:?}", r),
            None => panic!("Unexpected error retrieving table row."),
        }
        match t1.get_col(1) {
            Some(r) => println!("{:?}", r),
            None => panic!("Unexpected error retrieving table column."),
        }
        t1.insert_row(1);
        println!("{}", t1);
        t1.insert_col(0);
        println!("{}", t1);
        t1.remove_row(1);
        println!("{}", t1);
        t1.remove_col(0);
        println!("{}", t1);
        t1.insert_row_val(1,
                          &vec!(
                              Cell::new_data(CellType::Integer(20)),
                              Cell::new_data(CellType::Float(4.5)),
                          ));
        println!("{}", t1);
        t1.insert_col_val(0,
                          &vec!(
                              Cell::new_data(CellType::String(str!("New 1"))),
                              Cell::new_data(CellType::String(str!("New 2"))),
                              Cell::new_data(CellType::String(str!("New 3"))),
                              Cell::new_data(CellType::String(str!("New 4"))),
                          ));
        println!("{}", t1);
        assert_eq!(t1.pos_of(Cell::new()), Some(Pos::at(3, 3)));
        assert_eq!(t1.pos_list(Cell::new()), Some(vec!(Pos::at(3, 3))));
        println!("{:?}", t1.to_vec());
        assert!(t1.contains_all(&t1.to_vec()));
        assert!(t1.contains(&Cell::new()));
        assert_eq!(t1.capacity(), t1.len());
        assert!(t1 == t1.clone());
        for i in t1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        t1[(3, 3)] = Cell::new_data(CellType::String(str!("Set")));
        assert_eq!(t1[(3, 3)], Cell::new_data(CellType::String(str!("Set"))));
        println!("{:?}", t1);
        t1.clear();
        assert!(t1.is_empty());
    }

    #[test]
    fn tree_test() {
        let mut tree1: Tree<i32, i8> = Tree::new();
        tree1.insert_at(None,kv!(400, 1));
        tree1.insert_at(Some(400), kv!(100, 2));
        tree1.insert_at(Some(400), kv!(200, 3));
        tree1.insert_at(Some(400), kv!(300, 4));
        tree1.insert_at(Some(400), kv!(500, 5));
        tree1.insert_at(Some(400), kv!(600, 6));
        tree1.insert_at(Some(100), kv!(10, 7));
        tree1.insert_at(Some(100), kv!(20, 8));
        tree1.insert_at(Some(200), kv!(110, 9));
        tree1.insert_at(Some(500), kv!(510, 10));
        println!("{:?}", tree1);
        assert_eq!(tree1.len(), 10);
        let mut tree2: Tree<i32, i8> = tree1.clone();
        tree2.clear();
        assert!(tree2.is_empty());
        assert_eq!(tree1[200], 3);
        tree1[300] = 14;
        assert_eq!(tree1[300], 14);
        tree1[300] = 4;
        for i in tree1.clone().into_iter() {
            print!("{}: {}, ", i.key, i.value);
        }
        println!();
        let mut trav = tree1.clone().into_trav();
        println!("Default Traversal (Inorder)");
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Level Order Traversal");
        trav.level_order();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Postorder Traversal");
        trav.postorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Preorder Traversal");
        trav.preorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        println!("Inorder Traversal");
        trav.inorder();
        while trav.has_next() {
            print!("{} ", trav.next()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        while trav.has_prev() {
            print!("{} ", trav.prev()
                .expect("Unexpected error retrieving next node from tree."));
        }
        println!();
        assert!(tree1 == tree1);
        assert!(tree1.capacity() >= 10);
        assert!(tree1.contains(&kv!(500, 5)));
        assert!(tree1.contains_all(&tree1.clone().to_vec()));
        assert!(tree1.exists(300));
        assert!(!tree1.exists(1000));
        assert_eq!(tree1.get(500), Some(&5));
        assert_eq!(tree1.get(1000), None);
        assert!(tree1.remove(200));
        for i in tree1.clone().into_iter() {
            print!("{}: {}, ", i.key, i.value);
        }
        println!();
        tree1.replace(kv!(510, 9));
        assert_eq!(tree1[510], 9);
        println!("Degree of 400: {}", tree1.degree_of(400));
        println!("Edges: {}", tree1.edges());
        assert!(tree1.is_neighbor(400, 300));
        println!("Breadth: {}", tree1.breadth());
        println!("Child Nodes of 400: {:?}", tree1.child_nodes(&400));
        println!("Depth of 500: {}", tree1.depth_of(&500));
        println!("Level of 500: {}", tree1.level_of(&500));
        println!("Diameter: {}", tree1.diameter());
        println!("Height: {}", tree1.height());
        println!("Height from 100: {}", tree1.height_from(&100));
        assert!(tree1.is_ancestor(&100, &400));
        assert!(tree1.is_descendant(&400, &100));
        assert_eq!(tree1.is_leaf(&500), tree1.child_nodes(&500).is_empty());
        assert!(tree1.is_sibling(&100, &300));
        assert_eq!(tree1.parent_node(&500), Some(&1));
        assert_eq!(tree1.root_node(), Some(&1));
        tree1.set_node(kv!(510, 10));
        assert_eq!(tree1[510], 10);
        let sub: Tree<i32, i8> = tree1.subtree(100);
        println!("{:?}", sub);
        println!("Width of Level 1: {}", tree1.width(1));
        let tree2: Tree<i32, i8> = Tree::from_vec(&tree1.clone().to_vec());
        assert_eq!(tree1, tree2);
        println!("Path: {:?}", tree1.path_of(400, 10));
    }

    #[test]
    fn vector_test() {
        let mut vec1: Vector<i8> = Vector::new();
        vec1.reserve(30);
        assert_eq!(vec1.capacity(), 30);
        vec1.append(0);
        vec1.append(1);
        vec1.append(2);
        vec1.append(3);
        vec1.append(4);
        vec1.prepend(-1);
        vec1.prepend_all(vec1.clone().to_vec());
        vec1.insert(6, 5);
        vec1.insert_all(6, vec1.clone().to_vec());
        vec1.shrink();
        assert_eq!(vec1.len(), vec1.capacity());
        let len: usize = vec1.len();
        vec1.extend(5, &0);
        assert_eq!(vec1.len(), len + 5);
        vec1.truncate(len);
        assert_eq!(vec1.len(), len);
        vec1.resize(20, &0);
        assert_eq!(vec1.len(), 20);
        println!("{:?}", vec1);
        for i in vec1.clone().into_iter() {
            print!("{} ", i);
        }
        println!();
        assert!(vec1 == vec1);
        vec1[3] = 29;
        assert_eq!(vec1[3], 29);
        vec1.sort();
        println!("{:?}", vec1);
        assert!(vec1.is_sorted());
        vec1.sort_rev();
        println!("{:?}", vec1);
        assert!(vec1.is_sorted_rev());
        assert!(vec1.contains(&29));
        assert!(vec1.contains_all(&vec1.clone().to_vec()));
        assert_eq!(vec1.get(0), Some(&29));
        assert_eq!(vec1.index_of(&29), Some(0));
        assert_eq!(vec1.index_list(&0)
                       .expect("Unexpected error retrieving index list from vector")
                       .len(), 3);
        assert_eq!(vec1.last_index_of(&-1), Some(vec1.len() - 1));
        vec1.set(0, &30);
        assert_eq!(vec1[0], 30);
        let slice1 = vec1.slice(3..6);
        assert_eq!(*slice1, [4, 4, 4]);
        vec1.remove(30);
        assert!(!vec1.contains(&30));
        vec1.remove_any(3);
        assert!(!vec1.contains(&3));
        vec1.remove_last(0);
        assert_eq!(vec1.last_index_of(&0), Some(vec1.len() - 4));
        let mut vec2: Vector<i8> = Vector::from_vec(&vec![0, 1, 2]);
        vec1.remove_all(vec2.clone().to_vec());
        assert!(!vec1.contains_all(&vec2.clone().to_vec()));
        vec2.clear();
        assert!(vec2.is_empty());
        let vec3: Vector<i8> = Vector::with_capacity(10);
        assert_eq!(vec3.capacity(), 10);
        let vec4: Vector<i8> = Vector::with_length(10, &0);
        assert_eq!(vec4.len(), 10);
        println!("Reversed: {:?}", vec1.reverse());
    }
}
