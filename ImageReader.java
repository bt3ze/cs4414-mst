//adapted from http://stackoverflow.com/questions/6524196/java-get-pixel-array-from-image

import java.awt.image.BufferedImage;
import java.awt.image.DataBufferByte;
import java.io.IOException;
import javax.imageio.ImageIO;

public class ImageReader {

   public static void main(String[] args) throws IOException {

      BufferedImage hugeImage = ImageIO.read(ImageReader.class.getResource("step_gradient.jpg"));

      long startTime = System.nanoTime();
      int[][] result = readRGB(hugeImage);
      long endTime = System.nanoTime();
      System.out.println(String.format("Processing time: %s", timeToString(endTime - startTime)));
      
      System.out.println(String.format("2D array of pixel data:"));
      System.out.println(array2DToString(result));
   }

   private static int[][] readRGB(BufferedImage image) {

      final byte[] pixels = ((DataBufferByte) image.getRaster().getDataBuffer()).getData();
      final int width = image.getWidth();
      final int height = image.getHeight();
      final boolean hasAlphaChannel = image.getAlphaRaster() != null;

      //use bitshifting to efficiently encode RGB(A) values for pixels
      int[][] result = new int[height][width];
      if (hasAlphaChannel) {
         final int pixelLength = 4;
         for (int pixel = 0, row = 0, col = 0; pixel < pixels.length; pixel += pixelLength) {
            int argb = 0;
            argb += (((int) pixels[pixel] & 0xff) << 24); // alpha
            argb += ((int) pixels[pixel + 1] & 0xff); // blue
            argb += (((int) pixels[pixel + 2] & 0xff) << 8); // green
            argb += (((int) pixels[pixel + 3] & 0xff) << 16); // red
            result[row][col] = argb;
            col++;
            if (col == width) {
               col = 0;
               row++;
            }
         }
      } else {
         final int pixelLength = 3;
         for (int pixel = 0, row = 0, col = 0; pixel < pixels.length; pixel += pixelLength) {
            int argb = 0;
            argb += -16777216; // 255 alpha
            argb += ((int) pixels[pixel] & 0xff); // blue
            argb += (((int) pixels[pixel + 1] & 0xff) << 8); // green
            argb += (((int) pixels[pixel + 2] & 0xff) << 16); // red
            result[row][col] = argb;
            col++;
            if (col == width) {
               col = 0;
               row++;
            }
         }
      }

      return result;
   }

   private static String timeToString(long nanoSecs) {
      int minutes    = (int) (nanoSecs / 60000000000.0);
      int seconds    = (int) (nanoSecs / 1000000000.0)  - (minutes * 60);
      int millisecs  = (int) ( ((nanoSecs / 1000000000.0) - (seconds + minutes * 60)) * 1000);


      if (minutes == 0 && seconds == 0)
         return millisecs + "ms";
      else if (minutes == 0 && millisecs == 0)
         return seconds + "s";
      else if (seconds == 0 && millisecs == 0)
         return minutes + "min";
      else if (minutes == 0)
         return seconds + "s " + millisecs + "ms";
      else if (seconds == 0)
         return minutes + "min " + millisecs + "ms";
      else if (millisecs == 0)
         return minutes + "min " + seconds + "s";

      return minutes + "min " + seconds + "s " + millisecs + "ms";
   }

   private static String array2DToString(int[][] array){
   	  int h = array.length;
   	  if(h == 0) return "";
   	  int w = array[0].length;
   	  if(w == 0) return "";
   	  System.out.println("dimensions " + h + "x" + w);

   	  String retVal = "";
   	  for(int row = h-4; row < h; row++){
   	  	for(int col = w-4; col < w; col++){
   	  		int rgb = array[row][col];
   	  		int red = (rgb >> 16) & 0x000000FF;
			int green = (rgb >> 8 ) & 0x000000FF;
			int blue = (rgb) & 0x000000FF;
   	  		retVal = retVal + "R" + red + "G" + green + "B" + blue + "\t";
   	  	}
   	  	retVal += "\n";
   	  }
   	  return retVal;
   }
}