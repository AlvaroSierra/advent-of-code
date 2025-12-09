#include <corecrt.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main()
{

  FILE *filepointer;
  errno_t error;
  char character;
  int ncolumns = 0;

  if ((error = fopen_s(&filepointer, "input.txt", "rb")) != 0)
  {
    return error;
  }

  fseek(filepointer, 0, SEEK_END);
  long size = ftell(filepointer);
  fseek(filepointer, 0, SEEK_SET);

  char *data = malloc(size + 1);
  size_t read = fread(data, 1, size, filepointer);
  fclose(filepointer);
  data[size] = '\0';

  // Get the number of columns
  do
  {
    character = data[ncolumns];
    ncolumns += 1;
  } while (character != '\n');

  int directions[8] = {-1, 1, ncolumns - 1, ncolumns, ncolumns + 1, -ncolumns + 1, -ncolumns, -ncolumns - 1};

  int score = 0;
  int count;
  int inx;
  bool changed = true;

  // printf("%d, %d\n", ncolumns, size);
  //
  // printf("%s\n", data);
  do {
    changed = false;

    for (int k = 0; k < size; k++)
    {
      count = 0;
      if (data[k] == '\n' || data[k] == '\r' || data[k] == '.') {
        continue;
      }

      for (int dk = 0; dk < 8; dk++)
      {
        inx = k + directions[dk];

        if (inx != 0 && (inx - ncolumns + 2) % (ncolumns) == 0)
        {
          continue;
        }
        if (inx != 0 && (inx - ncolumns + 1) % (ncolumns) == 0)
        {
          continue;
        };
        if (inx < 0)
        {
          continue;
        }
        if (inx > size)
        {
          continue;
        }

        if (data[inx] == '@')
        {
          count++;
        } 
      }

      if (count < 4)
      {
        changed = true;
        data[k] = '.';
        score++;
      }
    }

  } while (changed);

  printf("Score: %d\n", score);
  printf("%s\n", data);

  free(data);
  return 0;
}
