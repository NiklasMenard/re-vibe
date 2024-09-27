import { useEffect, useRef } from 'react';

interface ImageScrollerProps {
  imagePaths: string[];
  className?: string;
}

const ImageScroller: React.FC<ImageScrollerProps> = ({ imagePaths, className }) => {
  const wrapperRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const wrapper = wrapperRef.current;

    if (wrapper) {
      const originalImages = Array.from(wrapper.children) as HTMLImageElement[];
      let loadedImages = 0;

      const handleImageLoad = () => {
        loadedImages += 1;
        if (loadedImages === originalImages.length) {
          // Once all images are loaded, clone and apply animation
          originalImages.forEach((img) => {
            const clone = img.cloneNode(true) as HTMLImageElement;
            wrapper.appendChild(clone);
          });

          const imageHeight = originalImages[0]?.clientHeight || 0;
          const totalImages = originalImages.length * 2;
          const totalHeight = imageHeight * totalImages;

          // Set the wrapper height to accommodate all images
          wrapper.style.height = `${totalHeight}px`;

          // Apply CSS animation for infinite scroll
          wrapper.style.animation = `scrollImages ${totalImages * 10}s linear infinite`;
        }
      };

      // Add load event listener to each image
      originalImages.forEach((img) => {
        img.addEventListener('load', handleImageLoad);
      });

      return () => {
        originalImages.forEach((img) => {
          img.removeEventListener('load', handleImageLoad);
        });
      };
    }
  }, []);

  return (
    <div className=" flex relative overflow-hidden flex-1">
      <div className={`image-wrapper ${className}`} ref={wrapperRef}>
        {imagePaths.map((path, index) => (
          <img key={index} src={path} alt={`Gallery image ${index + 1}`} className="p-2" />
        ))}
      </div>
    </div>
  );
};

export default ImageScroller;
