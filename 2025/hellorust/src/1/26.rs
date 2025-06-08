//스택 할당 vs. 힙 할당의 차이점
//스택 (Stack): 함수 호출, 지역 변수 등 프로그램의 짧은 수명 데이터를 저장하는 데 사용됩니다. 
//메모리 할당 및 해제가 빠르지만, 크기가 제한적입니다. 큰 데이터를 스택에 할당하려 하면 "스택 오버플로우"가 발생할 수 있습니다.
//힙 (Heap): 프로그램의 긴 수명 데이터를 저장하는 데 사용됩니다. 스택보다 느리지만, 더 유연하고 큰 메모리 할당이 가능합니다. Box<T>와 같은 스마트 포인터를 통해 힙에 데이터를 할당합니다.
fn main() {
    // 1. 스택에 큰 배열 할당 시도 (주의: 스택 오버플로우 가능성 있음)
    // 이 코드는 실제 실행 시 문제가 될 수 있습니다.
    // Rust Playground에서는 스택 크기 제한 때문에 컴파일/실행이 안 될 수 있습니다.
    // 로컬 환경에서도 기본 스택 크기를 초과하면 런타임 에러가 발생합니다.
    println!("--- 스택에 큰 배열 할당 시도 (주의) ---");
    const LARGE_SIZE_STACK: usize = 1_000_000; // 100만 개의 i32 (약 4MB)
    let stack_array: [i32; LARGE_SIZE_STACK] = [0; LARGE_SIZE_STACK];
    println!("스택 배열의 첫 번째 값: {}", stack_array[0]);
    println!("스택 배열의 크기: {} 바이트", std::mem::size_of_val(&stack_array));
    // 이 부분은 주석 처리하여 실제로 스택 오버플로우를 발생시키지 않도록 합니다.
    // 하지만 개념적으로 매우 큰 데이터는 스택에 직접 할당하지 않는다는 것을 보여줍니다.

    println!("\n--- Box를 사용하여 힙에 큰 배열 할당 ---");

    // 2. Box를 사용하여 힙에 큰 배열 할당
    const LARGE_SIZE_HEAP: usize = 10_000_000; // 1천만 개의 i32 (약 40MB)
    println!("{} 크기의 i32 배열을 힙에 할당합니다...", LARGE_SIZE_HEAP);

    // Box::new는 힙에 메모리를 할당하고 그 위에 데이터를 생성합니다.
    // [0; LARGE_SIZE_HEAP]은 배열을 초기화합니다.
    let heap_array_box: Box<[i32; LARGE_SIZE_HEAP]> = Box::new([0; LARGE_SIZE_HEAP]);

    println!("힙에 할당된 배열의 첫 번째 값: {}", heap_array_box[0]);
    println!("힙에 할당된 배열의 크기: {} 바이트", std::mem::size_of_val(&*heap_array_box));
    // 참고: `&*heap_array_box`는 `Box` 안의 실제 `[i32; ...]` 데이터에 대한 참조를 얻습니다.
    // `std::mem::size_of_val`은 이 참조가 가리키는 값의 크기를 측정합니다.

    // heap_array_box는 `Box<[i32; LARGE_SIZE_HEAP]>` 타입입니다.
    // 이 Box 자체는 스택에 있지만, Box가 가리키는 실제 데이터는 힙에 있습니다.
    // Box가 스코프를 벗어나면 (함수 끝 등), Rust는 자동으로 힙 메모리를 해제합니다.
    println!("`heap_array_box` 변수는 스코프를 벗어나면 자동으로 힙 메모리를 해제합니다.");

    // Box는 데이터를 소유하므로, 다른 곳으로 이동(move)할 수 있습니다.
    let another_box = heap_array_box; // 소유권 이동
    println!("소유권이 이동된 후 `another_box`의 첫 번째 값: {}", another_box[0]);

    // 이제 `heap_array_box`는 더 이상 유효하지 않습니다. (소유권이 `another_box`로 이동했기 때문)
    // println!("{}", heap_array_box[0]); // 에러 발생: use of moved value: `heap_array_box`
}