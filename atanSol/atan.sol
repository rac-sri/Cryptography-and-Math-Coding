
import "./libraries/wayray.sol";

pragma solidity >0.8.0;

contract ArcTan  {
    
    using WadRayMath for uint256;

    // taylor series: arctan(x) = x - x**3/x + x**5/5 - x**7/7.....
    
    int256 constant Wad = 1e18;
    int256 constant TaylorSeriesVariableCount = 2;
    
    // value in WAD
    function arcTan (int256 value) public returns (int256 ){

        int256 answer = 0;
        
        for( int256 i = 0; i<TaylorSeriesVariableCount; i++ ){
            int256 n = i *2 + 1;   // the power , denominator
            int256 c = i%2 ==0 ? int256(1):int256( -1);  // coefficient
            int256 cc = c == 1? int256(1): int256(-1);
            int256 xc = value > 0? int256(1): cc;
            uint256 term =  pow(uint256(value),uint256(n)) / uint256(n);
            answer += c * xc * int256(term); 
            
        }
        
       // answer /= Way;
        
        return answer;
    }
    
    
    function pow(uint256 val, uint256 p) internal returns (uint256) {
        
        uint256 finalVal = val;
        
        for(uint256  x = 1; x<p; x++){
            finalVal = finalVal.wadMul(val);
        }
        return finalVal;
    }
    
    //  function u_pow(int256 x, int256 p) internal pure returns (int256) {
    //     if (p == 0) return 1;
    //     if (p % 2 == 1) {
    //         return u_pow(x, p - 1).wadMul(x);
    //     } else {
    //         return u_pow(x, p / 2).wadMul(u_pow(x, p / 2));
    //     }
    // }

    // function pow(int256 x, int256 p) internal pure returns (int256) {
    //     int256 r = int256(u_pow(abs(x), p));
    //     if (p % 2 == 1) {
    //         return -1 * r;
    //     }
    //     return (r);
    // }
    
    
    // function abs(int256 x) internal pure returns (int256) {
    //     if (x < 0) {
    //         return int256(-x);
    //     }
    //     return int256(x);
    // }

}


