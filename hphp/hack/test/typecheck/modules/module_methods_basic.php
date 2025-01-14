//// modules.php
<?hh
<<file:__EnableUnstableFeatures('modules')>>

new module A {}
new module B {}
//// A.php
<?hh
// Copyright (c) Facebook, Inc. and its affiliates. All Rights Reserved.
<<file:__EnableUnstableFeatures('modules')>>
module A;

class A {
  internal function f(): void {}

  public function g(): void { $this->f(); /* ok */ }

}

function a(A $a): void { $a->f(); /* ok */ }

//// B.php
<?hh
// Copyright (c) Facebook, Inc. and its affiliates. All Rights Reserved.
<<file:__EnableUnstableFeatures('modules')>>
module B;

class B {
  public function f(A $a): void {
    $a->f(); // error
  }
}

function b(A $a): void { $a->f(); /* error */ }

//// main.php
<?hh
function main(A $a): void { $a->f(); /* error */ }
