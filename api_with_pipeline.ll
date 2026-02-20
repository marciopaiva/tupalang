@.str0 = private unnamed_addr constant [6 x i8] c"alive\00"
declare i32 @puts(i8*)

define i64 @health_check() {
entry:
  %t0 = getelementptr inbounds [6 x i8], [6 x i8]* @.str0, i64 0, i64 0
  call i32 @puts(i8* %t0)
  ret i64 200
}
