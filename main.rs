use rand::Rng;

struct Heap
{
    nodes: Vec<isize>,
}

impl Heap
{
    fn new() -> Heap
    {
        Heap {nodes: Vec::new(),}
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn parent(i: usize) -> Option<usize> {
        if i == 0
        {
            None
        }
        else
        {
            Some((i - 1) / 2)
        }
    }

    // Heaps an array up
    fn heapify_up(&mut self, mut index: usize) {
        while let Some (parent) = Heap::parent(index){
            if self.nodes[index] > self.nodes[parent] {
                self.nodes.swap(index, parent);
                index = parent;
            }
            else {
                break;
            }
        }
    }

    // Heaps an array down 
    fn heapify_down(&mut self, mut index: usize) {
        let size = self.nodes.len();
        loop 
        {
            let left = Heap::left(index);
            let right = Heap::right(index);

            let mut largest = index;
            if left < size && self.nodes[left] > self.nodes[largest] {
                largest = left;
            }
            if right < size && self.nodes[right] > self.nodes[largest] {
                largest = right;
            }
            if largest != index {
                self.nodes.swap(index, largest);
                index = largest;
                } else {
                    break;
            }
        }
    }

    // Inserts the array into the heap
    fn insert(&mut self, node: isize) {
        self.nodes.push(node);
        let last = self.nodes.len() - 1;
        self.heapify_up(last);
    }

    // Gets the mex array 
    fn get_max(&mut self) -> Option<isize> {
        if self.nodes.is_empty() {
            return None;
        }
        let max = self.nodes[0];
        let last = self.nodes.pop().unwrap();
        if !self.nodes.is_empty() {
            self.nodes[0] = last;
            self.heapify_down(0);
        }
        Some(max)
    }

    // Sorts the heap
    fn heap_sort(&mut self) -> Vec<isize> {
        let mut sorted = Vec::new();
        while let Some(max) = self.get_max() {
            sorted.push(max);
            println!("{:?}", self.nodes);
        }
        sorted.reverse();
        sorted
    }

    // Binary tree visual
    fn print_tree(values: &[isize]) 
    {
        let mut root = 0;
        let mut height = 1;
        let mut width = 64;

        while root < values.len()
        {
            let tree = (root + height).min(values.len());
            print!("{:w$}", "", w = width);
           
            // prints the values
            for i in root..tree
            {
                print!("{:^6}", values[i]);
                print!("{:w$}", "", w = width / 2);
            }
            println!();
                // prints the branches
            if tree < values.len()
            {
                print!("{:w$}", "", w = width);
                for _ in root..tree
                {
                    print!("  |  ");
                    print!("{:w$}", "", w = width / 2);
                }

                println!();
                print!("{:w$}", "", w = width);

                for _ in root..tree
                {
                    print!("---^---");
                    print!("{:w$}", "", w = width / 2);
                }
                println!();
                
            }
            println!();
            root = tree;
            height *= 2;
            if width > 1
            {
                width /= 2;
            }
            println!();
        }
    }

}

fn main() {

    let mut heap = Heap::new();
    let mut rng = rand::thread_rng();
    let mut nodes = [0isize; 31];

    for i in 0..31 
    {
        loop
        {
            let arr = rng.gen_range(10..99);
            if !nodes[..i].contains(&arr) {
                nodes[i] = arr;
                break;
            }
        }
    }

    for &node in nodes.iter() {
        heap.insert(node);
    }
    //let left = Heap::left(0);
    //let right = Heap::right(0);
    let mut input = String::new();
    println!("Unsorted Tree");
    Heap::print_tree(&heap.nodes);
    println!("Press Enter to start heap");
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{:?}", heap.nodes);
    //println!("Root: {}", heap.nodes[0]);

    let sort = heap.heap_sort();
    println!("\nSorted Array: {:?}", sort);

    println!("\nPress Enter again to see the binary tree");
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Sorted Tree");
    Heap::print_tree(&sort);
}
