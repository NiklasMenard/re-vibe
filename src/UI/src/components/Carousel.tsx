import useSwipe from '@/hooks/useSwipe';
import React, { useLayoutEffect, useState } from 'react';

import { ArrowButton } from './Buttons';

interface CarouselProps {
  renderCards?: boolean;
  renderOverlays?: boolean;
  onNextClick?: (index: number) => void;
  onPrevClick?: (index: number) => void;
  nextButtonText?: React.ReactNode;
  prevButtonText?: React.ReactNode;
  initialIndex?: number;
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

const Carousel: React.FC<CarouselProps> = ({
  renderCards = false,
  renderOverlays = true,
  onNextClick,
  onPrevClick,
  prevButtonText,
  nextButtonText,
  initialIndex = 0,
  children,
}) => {
  const [currentIndex, setCurrentIndex] = useState<number>(initialIndex);

  const prevSlide = (index: number): void => {
    const newIndex = Math.max(index - 1, 0);

    setCurrentIndex(newIndex);

    if (onPrevClick) {
      onPrevClick(index);
    }
  };

  const nextSlide = (index: number): void => {
    const newIndex = Math.min(index + 1, children.length - 1);

    setCurrentIndex(newIndex);

    if (onNextClick) {
      onNextClick(index);
    }
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

  const handleSwipe = (direction: 'left' | 'right', index: number) => {
    if (direction === 'right') {
      prevSlide(index); // Move to the previous slide
    } else if (direction === 'left') {
      nextSlide(index); // Move to the next slide
    }
  };

  const { onTouchEnd, onTouchStart } = useSwipe(handleSwipe);

  const animatePosition = (index: number) => {
    if (index === currentIndex) {
      return 'translate-x-0 scale-110 opacity-100'; // Center card
    }

    if (index === currentIndex + 1 || index === currentIndex - 1) {
      return `${index > currentIndex ? 'translate-x-[70%]' : '-translate-x-[70%]'} scale-75`;
    }

    if (index > currentIndex + 1 || index < currentIndex - 1) {
      return `${index > currentIndex ? 'translate-x-[70%]' : '-translate-x-[70%]'} scale-50`;
    }
  };

  const [visible, setVisible] = useState(false);

  useLayoutEffect(() => {
    setVisible(true);
  }, []);

  return (
    <div className="flex justify-center items-center relative flex-1 px-10 min-h-[85vh]">
      <ArrowButton
        onClick={() => prevSlide(currentIndex)}
        direction="left"
        className=" position absolute bottom-10 xl:bottom-[50%] left-10"
      >
        {prevButtonText}
      </ArrowButton>
      <div className="relative flex-grow flex items-center ">
        <div
          className={`flex items-center justify-center flex-1 transition-opacity duration-300 ${
            visible ? 'opacity-100' : 'opacity-0'
          }`}
        >
          {renderCards && children.length === 0 ? (
            <p>No products</p>
          ) : (
            children.map((card, index) => (
              <CardWrapper
                key={index}
                style={{
                  zIndex: calculateZIndex(index),
                }}
                onTouchStart={onTouchStart}
                onTouchEnd={(e) => onTouchEnd(e, currentIndex)}
                className={`${animatePosition(index)}`}
              >
                {card}
                {renderOverlays && generateOverlay(currentIndex, index)}
              </CardWrapper>
            ))
          )}
        </div>
      </div>
      <ArrowButton
        onClick={() => nextSlide(currentIndex)}
        className=" position absolute bottom-10 xl:bottom-[50%] right-10"
      >
        {nextButtonText}
      </ArrowButton>
    </div>
  );
};

export default Carousel;
