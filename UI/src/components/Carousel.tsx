import useSwipe from '@/hooks/useSwipe';
import React, { useState } from 'react';
import { Skeleton } from './Skeleton';
import { ArrowButton } from './Buttons';

interface CarouselProps {
  loading?: boolean;
  renderOverlays?: boolean;
  children: React.ReactNode[];
}

const LeftToRightOverlay = () => {
  return (
    <div
      className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-r from-black/5 to-black/70 rounded-[1rem]"
      aria-hidden="true"
    ></div>
  );
};

const RightToLeftOverlay = () => {
  return (
    <div
      className="absolute top-0 left-0 right-0 bottom-0 bg-gradient-to-l from-black/10 to-black/70 rounded-[1rem]"
      aria-hidden="true"
    ></div>
  );
};

const generateOverlay = (currentIndex: number, index: number) => {
  if (index === currentIndex) {
    return null;
  }

  if (index < currentIndex) {
    return <LeftToRightOverlay />;
  }

  if (index > currentIndex) {
    return <RightToLeftOverlay />;
  }
};

interface CardWrapperProps {
  index?: number;
  currentIndex?: number;
  style?: React.CSSProperties;
  onTouchStart?: (e: React.TouchEvent<HTMLDivElement>) => void;
  onTouchEnd?: (e: React.TouchEvent<HTMLDivElement>) => void;
  className?: string;
  children: React.ReactNode;
}

const CardWrapper: React.FC<CardWrapperProps> = ({ children, className, ...props }) => {
  return (
    <div
      className={`absolute flex items-center justify-center transition-transform duration-300 ease-linear ${className}`}
      {...props}
    >
      {children}
    </div>
  );
};

const Carousel: React.FC<CarouselProps> = ({ loading, renderOverlays, children }) => {
  const [currentIndex, setCurrentIndex] = useState<number>(2);

  const prevSlide = (): void => {
    setCurrentIndex((prevIndex) => (prevIndex === 0 ? 0 : prevIndex - 1));
  };

  const nextSlide = (): void => {
    setCurrentIndex((prevIndex) =>
      prevIndex === children.length - 1 ? children.length - 1 : prevIndex + 1
    );
  };

  const calculateZIndex = (index: number) => {
    if (index === currentIndex) {
      return children.length;
    }

    if (index < currentIndex) {
      return index;
    }

    if (index > currentIndex) {
      return children.length - 1 - index;
    }
  };

  const handleSwipe = (direction: 'left' | 'right') => {
    if (direction === 'right') {
      nextSlide();
    } else if (direction === 'left') {
      prevSlide();
    }
  };

  const { swipeStyle, onTouchEnd, onTouchStart } = useSwipe(handleSwipe);

  const animatePosition = (index: number) => {
    // Center card
    if (index === currentIndex) {
      return 'translate-x-0 scale-110 opacity-100';
    }

    // Cards to the left of current card
    if (index < currentIndex) {
      return 'translate-x-[65%] scale-75 '; // Move to left
    }

    // Cards to the right of current card
    if (index > currentIndex) {
      return '-translate-x-[65%] scale-75 '; // Move to right
    }
  };

  return (
    <div className="h-[60svh]">
      <div className="flex h-full flex-col px-14 overflow-hidden">
        <div className="relative w-full h-full flex items-center">
          <div className="flex flex-col h-full justify-end lg:justify-center ">
            <ArrowButton onClick={prevSlide} direction="left" className="touch-hidden" />
          </div>

          <div className="flex items-center justify-center flex-grow">
            {loading
              ? // Render skeletons if loading
                [...Array(4)].map((_, index) => (
                  <CardWrapper key={index} className={`${animatePosition(index)}`}>
                    <Skeleton className="h-80 w-64 md:h-[40rem] md:w-[40rem] rounded-[1rem]" />
                  </CardWrapper>
                ))
              : // Render cards when not loading
                children.map((card, index) => (
                  <CardWrapper
                    key={index}
                    style={{
                      zIndex: calculateZIndex(index),
                      ...swipeStyle,
                    }}
                    onTouchStart={onTouchStart}
                    onTouchEnd={onTouchEnd}
                    className={`${animatePosition(index)}`}
                  >
                    {card}
                    {renderOverlays && generateOverlay(currentIndex, index)}
                  </CardWrapper>
                ))}
          </div>

          <div className="flex flex-col h-full justify-end lg:justify-center">
            <ArrowButton onClick={nextSlide} className="touch-hidden" />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Carousel;
