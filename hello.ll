@.str0 = private unnamed_addr constant [6 x i8] c"Hello\00"
declare i32 @puts(i8*)

define void @main() {
entry:
  %t0 = alloca i64
  store i64 10, i64* %t0
  %t1 = load i64, i64* %t0
  %t2 = icmp sgt i64 %t1, 5
  br i1 %t2, label %if.then0, label %if.else1
if.then0:
  %t3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str0, i64 0, i64 0
  call i32 @puts(i8* %t3)
  br label %if.end2
if.else1:
  br label %if.end2
if.end2:
  ret void
}
