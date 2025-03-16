'use client';

import React, { useEffect, useRef, useState } from 'react';
import { motion } from 'framer-motion';

const TechWaveBackground = () => {
  const canvasRef = useRef(null);
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    setIsMounted(true);
    
    // Only run canvas code after component is mounted
    if (!isMounted) return;
    
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    let animationFrameId;
    
    // Set canvas size to match window
    const handleResize = () => {
      if (canvas) {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
      }
    };
    
    window.addEventListener('resize', handleResize);
    handleResize();
    
    // Wave parameters
    const waves = [
      { wavelength: 200, amplitude: 50, speed: 0.02, color: 'rgba(138, 43, 226, 0.2)' }, // Purple
      { wavelength: 150, amplitude: 30, speed: 0.03, color: 'rgba(75, 0, 130, 0.2)' },   // Indigo
      { wavelength: 100, amplitude: 20, speed: 0.04, color: 'rgba(123, 104, 238, 0.2)' } // Violet
    ];
    
    let time = 0;
    
    // Draw wave function
    const drawWaves = () => {
      if (!canvas || !ctx) return;
      
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      // Create gradient background
      const gradient = ctx.createLinearGradient(0, 0, 0, canvas.height);
      gradient.addColorStop(0, '#0a0a2a');
      gradient.addColorStop(1, '#1a1a4a');
      ctx.fillStyle = gradient;
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      
      // Draw each wave
      waves.forEach(wave => {
        ctx.beginPath();
        
        // Starting point of the wave
        ctx.moveTo(0, canvas.height / 2);
        
        // Draw wave points
        for (let x = 0; x < canvas.width; x++) {
          const y = Math.sin(x / wave.wavelength + time * wave.speed) * wave.amplitude + canvas.height / 2;
          ctx.lineTo(x, y);
        }
        
        // Complete the wave path to fill the bottom
        ctx.lineTo(canvas.width, canvas.height);
        ctx.lineTo(0, canvas.height);
        ctx.closePath();
        
        // Fill the wave
        ctx.fillStyle = wave.color;
        ctx.fill();
      });
      
      // Increment time for animation
      time += 1;
      
      // Create animation loop
      animationFrameId = requestAnimationFrame(drawWaves);
    };
    
    // Start the animation
    drawWaves();
    
    // Cleanup
    return () => {
      window.removeEventListener('resize', handleResize);
      if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
      }
    };
  }, [isMounted]);

  // Provide a simpler fallback background for before canvas is ready
  return (
    <div className="absolute inset-0 -z-10">
      {/* Fallback gradient background */}
      <div className="absolute inset-0 bg-gradient-to-b from-[#0a0a2a] to-[#1a1a4a]"></div>
      
      {/* Canvas for wave animation */}
      <canvas ref={canvasRef} className="absolute inset-0" />
      
      {/* Additional gradient overlay */}
      <div className="absolute inset-0 bg-gradient-to-b from-[#0a0a2a] via-transparent to-transparent opacity-70"></div>
    </div>
  );
};

export default TechWaveBackground;