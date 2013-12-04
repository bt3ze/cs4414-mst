import java.awt.image.BufferedImage;
import java.awt.image.WritableRaster;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.net.URL;
import java.util.Scanner;
import java.util.Vector;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.awt.image.DataBufferByte;

import javax.imageio.ImageIO;


public class MSTVisualizer {

	public static void main(String[] args) {
		BufferedImage image = null;


		File output = new File("out.jpg");
		//File output2 = new File("C:\\out.gif");
		//File output3 = new File("C:\\out.png");

		float [] pixelArray = null;
		Vector<Edge> edges = new Vector<Edge>();


		try {

			//Read and parse input
			File file = new File(args[0]);
			System.out.println("File opened: "+ file.getPath());
			//File file = new File("sample.out.txt");
			Scanner reader = new Scanner(new FileInputStream(file));
			int height = 0; 
			int width = 0;



			int malformedLinesSkipped = 0;
			int edgesAdded = 0;
			while (reader.hasNextLine()) {      
				String line = reader.nextLine();
				//Parse out the image dimensions
				if(line.contains("H") && line.contains("W")){
					int hpos = line.indexOf("H");
					int wpos = line.indexOf("W");
					height = Integer.parseInt(line.substring(hpos+1,wpos).trim());
					width = Integer.parseInt(line.substring(wpos+1).trim());
				}

				//Parse out all well-formed edge data into edges[]
				if(line.startsWith("(")){ //(0,0),(1,0):0
					String coords = line.split(":")[0];
					String [] components = coords.split(",");
					try{
						int startx = Integer.parseInt(components[0].substring(1));
						int starty = Integer.parseInt(components[1].substring(0,components[1].length()-1));
						int endx = Integer.parseInt(components[2].substring(1));
						int endy = Integer.parseInt(components[3].substring(0,components[3].length()-1));
						float cost = Float.parseFloat(line.split(":")[1]);

						edges.add(new Edge(startx,starty,endx,endy,cost));
						edgesAdded++;
					}
					catch(Exception e){
						malformedLinesSkipped++;
					}
				}
			}

			System.out.println("Edges added: "+edgesAdded);
			System.out.println("Malformed lines skipped: "+malformedLinesSkipped);


			//Testing
			/*URL url = new URL("http://www.mkyong.com/image/mypic.jpg");
			image = ImageIO.read(url);
			byte imageData[] = ((DataBufferByte)image.getData().getDataBuffer()).getData();
			int numPix = 0;
			for(int i = 0; i < imageData.length; i+=3){
				byte colorNum = imageData[i];
				int r = (int)imageData[i];
				int g = (int)imageData[i+1];
				int b = (int)imageData[i+2];
				System.out.println("R "+r+" G "+g+" B "+b);
				numPix++;
			}
			System.out.println("Dimensions yield "+image.getWidth()*image.getHeight() + " pixels");
			System.out.println("NumPix expected from file: "+(float)imageData.length/3);
			System.out.println("NumPix actual: "+numPix);*/
			//System.out.println(image.getType());

			//Initialize output image buffer
			int imageType = 5;
			image = new BufferedImage(width, height, imageType);
			WritableRaster wr = (WritableRaster) image.getData();
			
			//Set all pixels initially to white
			System.out.println("H"+height+"W"+width);
			for(int h = 0; h < height; h++){
				for(int w = 0; w < width; w++){
					wr.setPixel(w, h, Color(255,255,255));
				}
			}
			
				
			//Search for heavy edges
			float heavyCutoff = (float) 50; //230.0;

			int heavyEdges = 0;
			for(int i = 0; i < edges.size(); i++){
				if(edges.get(i).getCost() > heavyCutoff){					
					int startX = edges.get(i).getStartX();
					int startY = edges.get(i).getStartY();
					int endX = edges.get(i).getEndX();
					int endY = edges.get(i).getEndY();
					
					wr.setPixel(startX, startY, Color(0,0,0));
					wr.setPixel(endX, endY, Color(255,0,0));
					
					heavyEdges++;
				}
			}
			System.out.println("Heavy edges found: "+heavyEdges);


			//Depth-first search to recolor pixels
			for(int i = 0; i < edges.size(); i++){
				//System.out.println(edges.get(i).toString());
			}
			
			


			//Write output to file
			pixelArray = wr.getPixels(0, 0, width, height, pixelArray);
			wr.setPixels(0, 0, width, height, pixelArray); 
			image.setData(wr);
			ImageIO.write(image, "jpg", output);
			//ImageIO.write(image, "gif", output2);
			//ImageIO.write(image, "png", output3);



		} catch (IOException e) {
			e.printStackTrace();
		}
		System.out.println("Done");
	}

	private static float[] Color(int r, int g, int b){
		return new float[]{(float)r,(float)g,(float)b};
	}

}
