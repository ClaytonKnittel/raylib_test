
#include <raylib.h>

int main(int argc, char* argv[]) {
  InitWindow(800, 600, "raylib test");

  SetTargetFPS(60);
  SetExitKey(KEY_NULL);

  while (!WindowShouldClose()) {
    BeginDrawing();

    Color white = { 245, 245, 245, 255 };
    ClearBackground(white);

    Color lgray = { 200, 200, 200, 255 };
    DrawText("Window!", 190, 200, 20, lgray);
    EndDrawing();
  }

  CloseWindow();

  return 0;
}
