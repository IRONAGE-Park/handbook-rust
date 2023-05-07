# References and Borrowing

## References

- 함수의 매개변수로 사용한 후 원본을 반환하지 않으면 원본 데이터의 할당이 해제되게 됩니다.
- 만약 어떠한 한 객체에 특정한 매개변수만을 사용하는 함수에서 이러한 상황이 반복되면, 튜플 등으로 리턴 값을 래핑해서 내보내야 하는 번거로움이 있습니다.
- 이러한 문제를 해결하기 위해 Rust 언어는 참조자를 사용합니다.

### 참조자(`&`)

- `&` 키워드를 사용하여 매개변수의 타입을 선언하면, 해당 함수는 매개변수로 참조값만 전달받을 수 있습니다.
- 참조를 사용하여 인수를 전달하게 되면, 기존과는 달리 소유권이 넘어가지 않습니다.
- 소유권이 넘어가지 않기 때문에 참조를 사용하는 함수 내에서는 해당 변수의 값을 변경할 수 없습니다.

### 변경 가능한 참조

- 참조자를 선언할 때에도 접두어로 `mut` 키워드를 사용하면 변경 가능한 참조를 선언할 수 있습니다.
- `mut` 키워드로 선언된 변수에만 해당 키워드를 할당할 수 있습니다. 즉, 변경 가능한 변수를 참조로 만들기 위해서는 관련 변수에 모두 `mut` 키워드가 포함되어야 합니다.
- 만약 이처럼 매번 `mut` 키워드를 작성하는 것이 귀찮다고 새로운 변수로 선언하더라도 사용할 수 없습니다.
  - 한 번에 두 번이상 가변으로 빌릴 수 없기 때문입니다.
- 이러한 메커니즘을 잘 사용해야 컴파일 시간에 `Race Condition`을 방지할 수 있습니다.

> `Race Condition`의 조건
> 
> - 두 개 이상의 포인터가 동시에 동일한 데이터에 액세스
> - 적어도 하나의 포인터가 데이터를 수정하는 데 사용
> - 데이터에 대한 액세스를 동기화하는 데 사용하는 메커니즘의 부재

- 유사하게 이미 불변 참조자가 존재하는 동안에는 `mut` 키워드로 가변 참조를 생성할 수 없습니다.
- 불변 참조를 사용하는 사용자는 값이 갑자기 변경될 것이라고 기대하지 않기 때문입니다.
- 그러나 데이터를 읽기만 하는 사람은 다른 데이터에 영향을 줄 수 없기 때문에 여러 번 참조자를 생성하는 것이 허용됩니다.

## Dangling References

- 포인터가 있는 언어에서는 해당 메모리에 대한 포인터를 유지하면서 일부 메모리를 해제하여 댕글링 포인터를 잘못 생성하기 쉽습니다.
- Rust 언어에서는 컴파일러가 댕글링 포인터를 생성하지 못하도록 막는 것을 보장합니다.
- 참조를 반환하려고 할 필요 없이, 그저 일반 값을 반환하면 됩니다.