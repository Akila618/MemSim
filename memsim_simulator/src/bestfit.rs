pub fn how_bestfit_works() -> String {
    let explanation = "
Best-Fit Algorithm: How It Works

Purpose:
- Assigns a process to the smallest free memory block that is large enough to accommodate it.
- Minimizes wasted space by leaving the least amount of free space (fragmentation) in the block.

Steps:
1. Scan all available memory blocks.
2. Identify blocks that can fit the process size.
3. Select the block with the smallest remaining free space after allocation.
4. Allocate the process to the selected block and update its status as allocated.
5. Record any internal fragmentation (remaining unused space in the block).

Advantages:
- Reduces internal fragmentation compared to other algorithms.
- Efficiently utilizes memory by prioritizing smaller free blocks.
    ";
    explanation.to_string()
}
