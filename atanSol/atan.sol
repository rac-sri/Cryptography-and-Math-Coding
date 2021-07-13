
import "./libraries/wayray.sol";

pragma solidity >0.8.0;

contract ArcTan  {
    
    using WadRayMath for uint256;

    // taylor series: arctan(x) = x - x**3/3 + x**5/5 - x**7/7.....
    
    // arctan(x) + arctan(1/x)  = { pi/2 if x>0 and -pi/2 if x<0 
    
    int256 constant Wad = 1e18;
    int256 constant TaylorSeriesVariableCount = 3;
    int256 pi = 3140000000000000000;
    
    // value in WAD
    function arcTan (int256 value) public pure returns (int256 ){

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
    
    function arcTanWrapper(int256 value) public view returns (int256){
        uint256 one = 1e18;
        if(value < (1*Wad)){
             int256 finalResult = arcTan(value);
             return finalResult;
        }
        else {
            value = int256(one.wadDiv(uint256(value)));
            int256 finalResult = arcTan(value);
            finalResult = pi/2 - finalResult;
            return finalResult;
        }
        
    }
    
    
    function pow(uint256 val, uint256 p) internal pure returns (uint256) {
        
        uint256 finalVal = val;
        
        for(uint256  x = 1; x<p; x++){
            finalVal = finalVal.wadMul(val);
        }
        return finalVal;
    }
    

    function calculateM (int256 price, int256 L )  public returns (int256 result) {
        int256 arcTanResult = arcTanWrapper(price);
        int256 piBy2 =  pi / 2;
        uint256 mul = uint256(arcTanResult).wadMul(uint256(L));
        result =  int256((mul).wadDiv(uint256(piBy2)));
    }
}

