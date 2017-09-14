#include <X11/Xlib.h>
#include <X11/X.h>
#include <X11/Xutil.h>

#include <stdio.h>
#include <vector>

int write_image(int width, int height, unsigned char *data, int data_length)
{
  const char *filename = "new.ppm";

  FILE * fp;
  // const char *comment = "# this is my new binary pgm file";

  fp = fopen(filename, "wb");
  fprintf(fp, "P6\n %d\n %d\n %d\n", width, height, 255);
  fwrite(data, data_length * 3, 1, fp);
  fclose(fp);

  printf("OK - file %s saved\n", filename);

  return 0;
}

int main()
{
   Display *display = XOpenDisplay(NULL);
   Window root = DefaultRootWindow(display);

   XWindowAttributes gwa;

   XGetWindowAttributes(display, root, &gwa);
   int width = gwa.width;
   int height = gwa.height;

   XImage *image = XGetImage(display,root, 0,0 , width,height,AllPlanes, ZPixmap);

   if (image == nullptr) { return 1; }

   std::vector<unsigned char> array = std::vector<unsigned char>(width * height * 3);
   unsigned long red_mask = image->red_mask;
   unsigned long green_mask = image->green_mask;
   unsigned long blue_mask = image->blue_mask;

   for (int x = 0; x < width; x++) {
      for (int y = 0; y < height ; y++) {
         unsigned long pixel = XGetPixel(image,x,y);

         unsigned char blue = (pixel & blue_mask);
         unsigned char green = ((pixel & green_mask) >> 8);
         unsigned char red = ((pixel & red_mask) >> 16);

         array[(x + width * y) * 3]   = red;
         array[(x + width * y) * 3+1] = green;
         array[(x + width * y) * 3+2] = blue;
      }
   }

   write_image(width, height, array.data(), array.size() * 3);

   return 0;
}
